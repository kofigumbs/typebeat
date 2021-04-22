struct Voices {
    enum class SampleType {
        file,
        liveThrough,
        liveRecord,
        livePlay,
    };

    struct Player {
        int age = 0;
        SampleType sampleType;
        float position;
        float increment;
        float* note;
        float* gate;
        Entries* entries;
        Samples::File* file;
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

    Voices(dsp* d, int count) : players(count) {
        ButtonSearchUI ui;
        for (auto& p : players) {
            p.dsp.reset(d->clone());
            p.dsp->init(SAMPLE_RATE);
            ui.find("gate", p.gate, p.dsp.get());
            ui.find("note", p.note, p.dsp.get());
        }
    }

    void allocate(SampleType sampleType, int note, Entries* entries, Samples::File* file) {
        auto p = bestPlayer(note, entries);
        for (auto& q : players)
            q.age++;
        p->age = 0;
        p->sampleType = sampleType;
        p->position = 0;
        p->increment = pow(2.0f, note/12.0f) / pow(2.0f, 69/12.0f);
        *p->note = note;
        *p->gate = 1;
        p->entries = entries;
        p->file = file;
        p->dsp->instanceClear();
    }

    void release(int note, Entries* entries) {
        for (auto& p : players)
            if (*p.note == note && p.entries == entries)
                *p.gate = 0;
    }

    void run(const float input, float& outputL, float& outputR) {
        for (auto& p : players) {
            if (p.entries == nullptr)
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
    std::vector<Player> players;

    Player* bestPlayer(int note, Entries* entries) {
        Player* best;
        int bestScore = -1;
        for (auto& p : players) {
            auto pScore = score(note, entries, p);
            if (pScore > bestScore) {
                best = &p;
                bestScore = pScore;
            }
        }
        return best;
    }

    int score(int note, Entries* entries, const Player& p) {
        auto age = std::min(p.age, 99);
        if (p.entries == nullptr)
            age *= 1000;
        if (p.file && p.position >= p.file->length && *p.gate == 0)
            age *= 100;
        return age;
    }

    void run(const float input, Buffer& output, Player& p) {
        switch (p.sampleType) {
            case SampleType::file:
            case SampleType::livePlay:
                playFile(output, p);
                return;
            case SampleType::liveThrough:
            case SampleType::liveRecord:
                output.l = output.r = input;
                return;
        }
    }

    void playFile(Buffer& output, Player& p) {
        auto i = int(p.position);
        if (p.position == i && p.position < p.file->length) {
            output.l = left(i, p.file);
            output.r = right(i, p.file);
            p.position += p.increment;
        }
        else if (i + 1 < p.file->length) {
            output.l = interpolate(p.position-i, left(i, p.file), left(i+1, p.file));
            output.r = interpolate(p.position-i, right(i, p.file), right(i+1, p.file));
            p.position += p.increment;
        }
    }

    float interpolate(float x, float a, float b) {
        return a + x*(b - a);
    }

    float left(int i, Samples::File* file) {
        return file->frames[file->stereo ? 2*i : i];
    }

    float right(int i, Samples::File* file) {
        return file->frames[file->stereo ? 2*i+1 : i];
    }
};
