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
        MapUI ui;
        Entries* entries;
        Samples::Sample* sample;
        std::unique_ptr<dsp> dsp;
    };

    struct SendEffect {
        float buffer[2];
        MapUI ui;
        std::unique_ptr<dsp> dsp;
    };

    Voices(Autosave* autosave, dsp* insert, int count) : data(count) {
        for (auto& v : data) {
            MapUI ui;
            v.dsp.reset(insert->clone());
            v.dsp->init(SAMPLE_RATE);
            v.dsp->buildUserInterface(&v.ui);
        }
        for (auto dsp : { create_reverb(), create_echo() }) {
            sendEffects.emplace_back();
            auto& sendEffect = sendEffects.back();
            sendEffect.dsp.reset(dsp);
            sendEffect.dsp->init(SAMPLE_RATE);
            sendEffect.dsp->buildUserInterface(&sendEffect.ui);
            for (auto& entry : sendEffect.ui.getMap())
                autosave->bind(entry.first, new Autosave::Number(*entry.second));
        }
    }

    void allocate(SampleType sampleType, int note, Entries* entries, Samples::Sample* sample) {
        auto sampleDetune = entries->find("sample:detune");
        auto v = bestVoice(note, entries);
        for (auto& q : data)
            q.age++;
        v->age = 0;
        v->position = 0;
        v->increment = pow(2, (note + sampleDetune->value/10)/12) / pow(2, 69.f/12);
        v->entries = entries;
        v->sample = sample;
        v->ui.setParamValue("gate", 1);
        v->ui.setParamValue("note", note);
        v->ui.setParamValue("live", sampleType == SampleType::LiveThrough || sampleType == SampleType::LiveRecord);
        v->dsp->instanceClear();
    }

    void release(int note, Entries* entries) {
        for (auto& v : data)
            if (v.ui.getParamValue("note") == note && v.entries == entries)
                v.ui.setParamValue("gate", 0);
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
        for (auto& sendEffect : sendEffects)
            stereoCompute(sendEffect.dsp, sendEffect.buffer, &output, 1);
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

    int score(int note, Entries* entries, Voice& v) {
        auto age = std::min(v.age, 99);
        if (v.entries == nullptr)
            age *= 1000;
        if (v.sample && v.position >= v.sample->length && v.ui.getParamValue("gate") == 0)
            age *= 100;
        return age;
    }

    void play(const float input, float* output, Voice& v) {
        if (v.ui.getParamValue("live")) {
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
