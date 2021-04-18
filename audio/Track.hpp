struct Track {
    static const int viewsPerPage = 4;
    static const int concertPitch = 69;
    static const int maxLiveRecordLength = 60*SAMPLE_RATE;

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
    Voices::SampleType sampleType = Voices::SampleType_file;
    Voices* voices;

    Track(Voices* v, Transport* t, Samples::File* s, Entries e) : voices(v), transport(t), sampleFile(s), entries(e), steps() {
        sampleLive.frames.reset(new float[maxLiveRecordLength]);
    }

    Entries::Control* entry(const std::string& name) {
        return entries.find(name);
    }

    bool control(const std::string& name, int& value) {
        auto control = entries.find(name);
        if (control != nullptr)
            value = control->value;
        return control != nullptr;
    }

    void run(const float input) {
        if (sampleType == Voices::SampleType_liveRecord && sampleLive.length < maxLiveRecordLength)
            sampleLive.frames[sampleLive.length++] = input;
        if (transport->newStep()) {
            auto& step = steps[transport->step % length];
            if (step.skipNext)
                step.skipNext = false;
            else if (step.active)
                keyDown(step.note);
        }
    }

    void setSampleType(int value) {
        sampleType = static_cast<Voices::SampleType>(value);
        if (sampleType == Voices::SampleType_liveRecord)
            sampleLive.length = 0;
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
        voices->release(note, &entries);
    }

  private:
    int pageStart = 0;
    int length = Transport::maxResolution*4;
    Transport* transport;
    Samples::File* sampleFile;
    Samples::File sampleLive;
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
        auto file = sampleType == Voices::SampleType_file ? sampleFile : &sampleLive;
        voices->allocate(sampleType, note, naturalNote, &entries, file);
    }
};
