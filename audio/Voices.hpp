struct Voices {
    enum Source {
        Source_sample,
        Source_input,
    };

    struct Player {
        Source source;
        int sample;
        float position;
        float increment;
        float* note;
        float* gate;
        Entries* entries;
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

    Voices(Samples* s, dsp* d, int count) : samples(s), players(count) {
        ButtonSearchUI ui;
        for (auto& p : players) {
            p.dsp.reset(d->clone());
            p.dsp->init(SAMPLE_RATE);
            ui.find("gate", p.gate, p.dsp.get());
            ui.find("note", p.note, p.dsp.get());
        }
    }

    void allocate(Source source, int sample, int note, int naturalNote, Entries* entries) {
        auto& p = players[nextVoice++ % players.size()];
        p.dsp->instanceClear();
        p.source = source;
        p.sample = sample;
        p.position = 0;
        p.increment = pow(2.0f, note / 12.0f) / pow(2.0f, naturalNote / 12.0f);
        *p.note = note;
        *p.gate = 1;
        p.entries = entries;
    }

    void release(int sample, int note) {
        for (auto& p : players)
            if (p.sample == sample && *p.note == note)
                *p.gate = 0;
    }

    void run(const float input, float& outputL, float& outputR) {
        for (auto& p : players) {
            if (!p.entries || !p.gate)
                continue;
            Buffer toDsp, fromDsp;
            run(input, toDsp, p);
            p.entries->prepareToWrite();
            p.dsp->buildUserInterface(p.entries);
            p.dsp->compute(
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
    Samples* samples;
    std::vector<Player> players;

    void run(const float input, Buffer& output, Player& p) {
        if (p.source == Source_input) {
            output.l = output.r = input;
            return;
        }
        samples->get(p.sample, [&](auto length, auto frames) {
            auto i = int(p.position);
            if (p.position == i && p.position < length) {
                output.l = left(i, frames);
                output.r = right(i, frames);
                p.position += p.increment;
            }
            else if (i + 1 < length) {
                output.l = interpolate(p.position-i, left(i, frames), left(i + 1, frames));
                output.r = interpolate(p.position-i, right(i, frames), right(i + 1, frames));
                p.position += p.increment;
            }
        });
    }

    float interpolate(float x, float a, float b) {
        return a + x*(b - a);
    }

    float left(int i, float* frames) {
        return frames[2*i];
    }

    float right(int i, float* frames) {
        return frames[2*i + 1];
    }
};
