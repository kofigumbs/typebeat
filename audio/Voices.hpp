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

    struct ButtonSearchUI : GenericUI {
        std::string target;
        float* result;

        void addButton(const char* label, float* zone) override {
            if (target == label)
                result = zone;
        }

        void find(const std::string t, float*& destination, dsp* dsp) {
            target = t;
            dsp->buildUserInterface(this);
            destination = result;
        }
    };

    Voices(int count, dsp* insert) : data(count) {
        ButtonSearchUI ui;
        for (auto& v : data) {
            v.dsp.reset(insert->clone());
            v.dsp->init(SAMPLE_RATE);
            ui.find("gate", v.gate, v.dsp.get());
            ui.find("note", v.note, v.dsp.get());
            ui.find("live", v.live, v.dsp.get());
        }
        reverb = createSend(create_reverb());
        delay = createSend(create_delay());
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
        float sample[2], reverbSend[2], delaySend[2];
        for (auto& v : data) {
            if (v.entries == nullptr)
                continue;
            play(input, sample, v);
            v.entries->prepareToWrite();
            v.dsp->buildUserInterface(v.entries);
            stereoCompute(v.dsp, sample, { output, reverbSend, delaySend });
        }
        stereoCompute(reverb, reverbSend, { output });
        stereoCompute(delay, delaySend, { output });
    }

  private:
    std::vector<Voice> data;
    std::unique_ptr<dsp> reverb;
    std::unique_ptr<dsp> delay;

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

    void stereoCompute(std::unique_ptr<dsp>& dsp, float* input, std::initializer_list<float*> outputs) {
        float* pInput[2];
        float* pOutput[outputs.size()*2];
        float buffer[outputs.size()*2];
        stereoMap(input, pInput);
        for (int i = 0; i < outputs.size(); i++)
            stereoMap(buffer + 2*i, pOutput + 2*i);
        dsp->compute(1, pInput, pOutput);
        int i = 0;
        for (auto& output : outputs) {
            output[0] += buffer[2 * i++];
            output[1] += buffer[2 * i++];
        }
    }

    void stereoMap(float* audio, float** pointers) {
        pointers[0] = audio;
        pointers[1] = audio + 1;
    }
};
