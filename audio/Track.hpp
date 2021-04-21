struct Track {
    static const int viewsPerPage = 4;
    static const int concertPitch = 69;
    static const int maxLiveRecordLength = 60*SAMPLE_RATE;

    enum class View {
        none,
        empty,
        exactlyOnStep,
        containsSteps,
    };

    struct Step {
        bool active;
        bool skipNext;
        int note = concertPitch;
    };

    int resolution = 4;
    int octave = 4;
    int naturalNote = concertPitch;
    Voices::SampleType sampleType = Voices::SampleType::file;
    Voices* voices;

    Track(Voices* v, Song* s, Samples::File* f, Entries e) : voices(v), song(s), sampleFile(f), entries(e), steps() {
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
        if (sampleType == Voices::SampleType::liveRecord && sampleLive.length < maxLiveRecordLength)
            sampleLive.frames[sampleLive.length++] = input;
        if (song->newStep()) {
            auto& step = steps[song->step % length];
            if (step.skipNext)
                step.skipNext = false;
            else if (step.active)
                keyDown(step.note);
        }
    }

    void setSampleType(int value) {
        sampleType = static_cast<Voices::SampleType>(value);
        if (sampleType == Voices::SampleType::liveRecord)
            sampleLive.length = 0;
    }

    int bars() {
        return std::ceil((float) length / Song::maxResolution);
    }

    int view(int i) {
        return static_cast<int>(viewFrom(viewIndexToStart(i)));
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
        if (resolution < Song::maxResolution)
            resolution *= 2;
    }

    void adjustLength(int diff) {
        int min = Song::maxResolution;
        length = std::clamp(length + diff*Song::maxResolution, min, (int) steps.size());
    }

    void toggle(int i) {
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View::none:
                return;
            case View::empty:
            case View::exactlyOnStep:
                steps[start].active ^= true;
                steps[start].skipNext = false;
                return;
            case View::containsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    steps[i].active = false;
                return;
        }
    }

    void play() {
        play(naturalNote);
    }

    void play(int note) {
        if (song->playing && song->armed) {
            auto quantizedStep = song->quantizedStep(resolution);
            steps[quantizedStep % length] = {
                .active = true,
                .skipNext = quantizedStep >= song->step,
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
    int length = Song::maxResolution*4;
    Song* song;
    Samples::File* sampleFile;
    Samples::File sampleLive;
    Entries entries;
    std::array<Step, Song::maxResolution*16*8> steps;

    int viewLength() {
        return Song::maxResolution / resolution;
    }

    int viewIndexToStart(int i) {
        return pageStart + i * viewLength();
    }

    View viewFrom(int start) {
        if (start >= length)
            return View::none;
        int countActive = 0;
        int lastActive = 0;
        for (int i = start; i < start + viewLength(); i++) {
            if (steps[i].active) {
                countActive++;
                lastActive = i;
            }
        }
        if (countActive == 0)
            return View::empty;
        else if (countActive == 1 && lastActive == start)
            return View::exactlyOnStep;
        else
            return View::containsSteps;
    }

    void keyDown(int note) {
        auto file = sampleType == Voices::SampleType::file ? sampleFile : &sampleLive;
        voices->allocate(sampleType, note, naturalNote, &entries, file);
    }
};
