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

    struct Buffer {
        float l = 0;
        float r = 0;
    };

    Voices(dsp* d, int count) : data(count) {
        ButtonSearchUI ui;
        for (auto& v : data) {
            v.dsp.reset(d->clone());
            v.dsp->init(SAMPLE_RATE);
            ui.find("gate", v.gate, v.dsp.get());
            ui.find("note", v.note, v.dsp.get());
            ui.find("live", v.live, v.dsp.get());
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

    void run(const float input, float& outputL, float& outputR) {
        for (auto& v : data) {
            if (v.entries == nullptr)
                continue;
            Buffer toDsp, fromDsp;
            run(input, toDsp, v);
            v.entries->prepareToWrite();
            v.dsp->buildUserInterface(v.entries);
            v.dsp->compute(
                1,
                (float**) (float*[]) { &toDsp.l, &toDsp.r },
                (float**) (float*[]) { &fromDsp.l, &fromDsp.r }
            );
            outputL += fromDsp.l;
            outputR += fromDsp.r;
        }
    }

  private:
    int nextVoice = 0;
    std::vector<Voice> data;

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

    void run(const float input, Buffer& output, Voice& v) {
        if (*v.live)
            output.l = output.r = input;
        else
            playSample(output, v);
    }

    void playSample(Buffer& output, Voice& v) {
        auto i = int(v.position);
        if (v.position == i && v.position < v.sample->length) {
            output.l = left(i, v.sample);
            output.r = right(i, v.sample);
            v.position += v.increment;
        }
        else if (i + 1 < v.sample->length) {
            output.l = interpolate(v.position-i, left(i, v.sample), left(i+1, v.sample));
            output.r = interpolate(v.position-i, right(i, v.sample), right(i+1, v.sample));
            v.position += v.increment;
        }
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
};
