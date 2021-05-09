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
        std::unique_ptr<dsp> dsp;
        MapUI ui;
        float buffer[2];
        SendEffect(class dsp* d) : dsp(d) {
            dsp->buildUserInterface(&ui);
        }
    };

    Voices(int count, dsp* insert) : data(count) {
        for (auto& v : data) {
            MapUI ui;
            v.dsp.reset(insert->clone());
            v.dsp->init(SAMPLE_RATE);
            v.dsp->buildUserInterface(&ui);
            v.gate = ui.getParamZone("gate");
            v.note = ui.getParamZone("note");
            v.live = ui.getParamZone("live");
        }
        sendEffects.emplace_back(create_reverb());
        sendEffects.emplace_back(create_delay());
    }

    void allocate(SampleType sampleType, int note, Entries* entries, Samples::Sample* sample) {
        auto sampleDetune = entries->find("sample:detune");
        auto v = bestVoice(note, entries);
        for (auto& q : data)
            q.age++;
        v->age = 0;
        v->position = 0;
        v->increment = pow(2, (note + sampleDetune->value/10)/12) / pow(2, 69.f/12);
        *v->note = note;
        *v->gate = 1;
        *v->live = sampleType == SampleType::LiveThrough || sampleType == SampleType::LiveRecord;
        v->entries = entries;
        v->sample = sample;
        v->dsp->instanceClear();
    }

    void release(int note, Entries* entries) {
        for (auto& v : data)
            if (*v.note == note && v.entries == entries)
                *v.gate = 0;
    }

    void run(const float input, float* output) {
        float sample[2];
        int busCount = sendEffects.size() + 1;
        float* buses[busCount];
        buses[0] = output;
        for (int i = 0; i < sendEffects.size(); i++) {
            sendEffects[i].buffer[0] = 0;
            sendEffects[i].buffer[1] = 0;
            buses[i+1] = sendEffects[i].buffer;
        }
        for (auto& v : data) {
            if (v.entries == nullptr)
                continue;
            play(input, sample, v);
            v.entries->prepareToWrite();
            v.dsp->buildUserInterface(v.entries);
            stereoCompute(v.dsp, sample, buses, busCount);
        }
        for (auto& sendEffect : sendEffects)
            stereoCompute(sendEffect.dsp, sendEffect.buffer, buses, 1);
    }

  private:
    std::vector<Voice> data;
    std::vector<SendEffect> sendEffects;

    std::unique_ptr<dsp> createSend(dsp* send) {
        send->init(SAMPLE_RATE);
        assert(send->getNumInputs() == 2);
        assert(send->getNumOutputs() == 2);
        return std::unique_ptr<dsp>(send);
    }

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

    int score(int note, Entries* entries, const Voice& v) {
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
