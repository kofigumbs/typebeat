struct Voices {
    enum class SampleType {
        File,
        LiveThrough,
        LiveRecord,
        LivePlay,
    };

    struct Voice {
        int age = 0;
        float position;
        float increment;
        float* note;
        float* gate;
        float* live;
        Entries* entries;
        Samples::Sample* sample;
        std::unique_ptr<dsp> dsp;
    };

    struct SendEffect {
        float buffer[2];
        std::unique_ptr<dsp> dsp;
    };

    Entries entries;

    Voices(Autosave* autosave, dsp* insert, int count) : data(count) {
        for (auto& v : data) {
            MapUI ui;
            v.dsp.reset(insert->clone());
            v.dsp->init(SAMPLE_RATE);
            v.dsp->buildUserInterface(&ui);
            v.note = ui.getParamZone("note");
            v.gate = ui.getParamZone("gate");
            v.live = ui.getParamZone("live");
        }
        for (auto dsp : { create_reverb(), create_echo() }) {
            assert(dsp->getNumInputs() == 2);
            assert(dsp->getNumOutputs() == 2);
            auto& sendEffect = sendEffects.emplace_back();
            sendEffect.dsp.reset(dsp);
            sendEffect.dsp->init(SAMPLE_RATE);
            sendEffect.dsp->buildUserInterface(&entries);
        }
        entries.bind("send.", autosave);
    }

    void allocate(SampleType sampleType, int note, Entries* entries, Samples::Sample* sample) {
        Entries::Entry* sampleDetune;
        entries->find("sample:detune", sampleDetune);
        auto v = bestVoice(note, entries);
        for (auto& q : data)
            q.age++;
        v->age = 0;
        v->position = 0;
        v->increment = pow(2, (note + sampleDetune->value/10)/12) / pow(2, 69.f/12);
        v->entries = entries;
        v->sample = sample;
        *v->gate = 1;
        *v->note = note;
        *v->live = sampleType == SampleType::LiveThrough || sampleType == SampleType::LiveRecord;
        v->dsp->instanceClear();
    }

    void release(int note, Entries* entries) {
        for (auto& v : data)
            if (*v.note == note && v.entries == entries)
                *v.gate = 0;
    }

    void run(const float input, float* output) {
        float sampleBuffer[2];
        int sendBufferCount = sendEffects.size() + 1;
        float* sendBuffers[sendBufferCount];
        sendBuffers[0] = output;
        for (int i = 0; i < sendEffects.size(); i++) {
            sendBuffers[i+1] = sendEffects[i].buffer;
            sendEffects[i].buffer[0] = sendEffects[i].buffer[1] = 0;
        }
        for (auto& v : data) {
            if (v.entries == nullptr)
                continue;
            play(input, sampleBuffer, v);
            v.entries->prepareToWrite();
            v.dsp->buildUserInterface(v.entries);
            stereoCompute(v.dsp, sampleBuffer, sendBuffers, sendBufferCount);
        }
        entries.prepareToWrite();
        for (auto& sendEffect : sendEffects) {
            sendEffect.dsp->buildUserInterface(&entries);
            stereoCompute(sendEffect.dsp, sendEffect.buffer, &output, 1);
        }
    }

  private:
    std::vector<Voice> data;
    std::vector<SendEffect> sendEffects;

    Voice* bestVoice(int note, Entries* entries) {
        Voice* best;
        int bestScore = -1;
        for (auto& v : data) {
            auto pScore = score(note, entries, v);
            if (pScore > bestScore) {
                best = &v;
                bestScore = pScore;
            }
        }
        return best;
    }

    int score(int note, Entries* entries, Voice& v) {
        auto age = std::min(v.age, 99);
        if (v.entries == nullptr)
            age *= 1000;
        if (v.sample && v.position >= v.sample->length && *v.gate == 0)
            age *= 100;
        return age;
    }

    void play(const float input, float* output, Voice& v) {
        if (*v.live) {
            output[0] = output[1] = input;
            return;
        }
        auto i = int(v.position);
        if (v.position == i && v.position < v.sample->length) {
            output[0] = left(i, v.sample);
            output[1] = right(i, v.sample);
            v.position += v.increment;
        }
        else if (i + 1 < v.sample->length) {
            output[0] = interpolate(v.position-i, left(i, v.sample), left(i+1, v.sample));
            output[1] = interpolate(v.position-i, right(i, v.sample), right(i+1, v.sample));
            v.position += v.increment;
        }
        else
            output[0] = output[1] = 0;
    }

    float interpolate(float x, float a, float b) {
        return a + x*(b - a);
    }

    float left(int i, Samples::Sample* sample) {
        return sample->frames[sample->stereo ? 2*i : i];
    }

    float right(int i, Samples::Sample* sample) {
        return sample->frames[sample->stereo ? 2*i+1 : i];
    }

    void stereoCompute(std::unique_ptr<dsp>& dsp, float* input, float** outputs, size_t outputCount) {
        float* inputPointers[2] { &input[0], &input[1] };
        float* outputPointers[outputCount][2];
        float outputBuffers[outputCount][2];
        for (int i = 0; i < outputCount; i++) {
            outputPointers[i][0] = &outputBuffers[i][0];
            outputPointers[i][1] = &outputBuffers[i][1];
        }
        dsp->compute(1, (float**) inputPointers, (float**) outputPointers);
        for (int i = 0; i < outputCount; i++) {
            outputs[i][0] += outputBuffers[i][0];
            outputs[i][1] += outputBuffers[i][1];
        }
    }
};
