struct Track {
    static const int viewsPerPage = 4;
    static const int concertPitch = 69;

    enum View {
        View_none,
        View_empty,
        View_exactlyOnStep,
        View_containsSteps,
    };

    struct Step {
        bool active;
        bool skipNext;
        int note = concertPitch;
    };

    int resolution = 4;
    int octave = 4;
    int naturalNote = concertPitch;
    Voices* voices;

    Track(int s, Voices* v, Transport* t, Entries e) : sample(s), voices(v), transport(t), entries(e), steps() {
    }

    Entries::Control* entry(const std::string& name) {
        for (auto& control : entries.data)
            if (name == control.label)
                return &control;
        return nullptr;
    }

    int control(const std::string& name) {
        for (const auto& control : entries.data)
            if (name == control.label)
                return control.value;
        return 0;
    }

    void run(const float input) {
        if (transport->newStep()) {
            auto& step = steps[transport->step % length];
            if (step.skipNext)
                step.skipNext = false;
            else if (step.active)
                keyDown(step.note);
        }
    }

    int bars() {
        return std::ceil(1.f * length / Transport::maxResolution);
    }

    int view(int i) {
        return viewFrom(viewIndexToStart(i));
    }

    int viewStart() {
        return pageStart / viewLength();
    }

    void movePage(int diff) {
        auto newPageStart = pageStart + diff * viewsPerPage * viewLength();
        if (newPageStart < length)
            pageStart = (std::max)(0, newPageStart);
    }

    void zoomOut() {
        if (resolution > 1) {
            resolution /= 2;
            pageStart = pageStart / viewLength() * viewLength();
        }
    }

    void zoomIn() {
        if (resolution < Transport::maxResolution)
            resolution *= 2;
    }

    void toggle(int i) {
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View_none:
                return;
            case View_empty:
            case View_exactlyOnStep:
                steps[start].active ^= true;
                steps[start].skipNext = false;
                return;
            case View_containsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    steps[i].active = false;
                return;
        }
    }

    void play() {
        play(naturalNote);
    }

    void play(int note) {
        if (transport->playing && transport->armed) {
            auto quantizedStep = transport->quantizedStep(resolution);
            steps[quantizedStep % length] = {
                .active = true,
                .skipNext = quantizedStep >= transport->step,
                .note = note 
            };
        }
        keyDown(note);
    }

    void release() {
        release(naturalNote);
    }

    void release(int note) {
        voices->release(sample, note);
    }

  private:
    int sample;
    int pageStart = 0;
    int length = Transport::maxResolution*4;
    Transport* transport;
    Entries entries;
    std::array<Step, Transport::maxResolution*16*8> steps;

    int viewLength() {
        return Transport::maxResolution / resolution;
    }

    int viewIndexToStart(int i) {
        return pageStart + i * viewLength();
    }

    View viewFrom(int start) {
        if (start >= length)
            return View_none;
        int countActive = 0;
        int lastActive = 0;
        for (int i = start; i < start + viewLength(); i++) {
            if (steps[i].active) {
                countActive++;
                lastActive = i;
            }
        }
        if (countActive == 0)
            return View_empty;
        else if (countActive == 1 && lastActive == start)
            return View_exactlyOnStep;
        else
            return View_containsSteps;
    }

    void keyDown(int note) {
        voices->allocate(Voices::Source_sample, sample, note, naturalNote, &entries);
    }
};
