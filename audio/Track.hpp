struct Track {
    static const int viewsPerPage = 4;
    static const int maxLiveRecordLength = 60*SAMPLE_RATE;

    enum class View {
        None,
        Empty,
        ExactlyOnStep,
        ContainsSteps,
    };

    struct Step {
        bool active;
        bool skipNext;
        int key;
    };

    bool mute = false;
    bool useKey = true;
    int lastKey = 12; // 440Hz (concert pitch A) in C Major
    int resolution = 4;
    int octave = 4;
    Voices::SampleType sampleType = Voices::SampleType::File;
    Voices* voices;

    Track(int id, Autosave* autosave, Voices* v, Samples* samples, Song* s, Entries e) : voices(v), song(s), defaultSample(&samples->data[id]), entries(e), steps() {
        liveSample.frames.reset(new float[maxLiveRecordLength]);
        auto prefix = "track[" + std::to_string(id) + "].";
        entries.bind(prefix, autosave);
        autosave->bind(prefix + "mute", new Autosave::Number(mute));
        autosave->bind(prefix + "useKey", new Autosave::Number(useKey));
        autosave->bind(prefix + "lastKey", new Autosave::Number(lastKey));
        autosave->bind(prefix + "resolution", new Autosave::Number(resolution));
        autosave->bind(prefix + "octave", new Autosave::Number(octave));
        autosave->bind(prefix + "length", new Autosave::Number(length));
        autosave->bind(prefix + "sampleType", new Autosave::Number(sampleType));
        autosave->bind(prefix + "steps:active", new Autosave::Array(steps, &Track::Step::active));
        autosave->bind(prefix + "steps:note", new Autosave::Array(steps, &Track::Step::key));
    }

    Entries::Entry* entry(const std::string& name) {
        return entries.find(name);
    }

    bool entry(const std::string& name, int& value) {
        auto entry = entries.find(name);
        if (entry != nullptr)
            value = entry->value;
        return entry != nullptr;
    }

    void run(const float input) {
        if (sampleType == Voices::SampleType::LiveRecord && liveSample.length < maxLiveRecordLength)
            liveSample.frames[liveSample.length++] = input;
        if (song->newStep()) {
            auto& step = steps[song->step % length];
            if (step.skipNext)
                step.skipNext = false;
            else if (step.active && !mute)
                restartVoice(step.key);
        }
    }

    void setSampleType(int value) {
        sampleType = static_cast<Voices::SampleType>(value);
        if (sampleType == Voices::SampleType::LiveRecord)
            liveSample.length = 0;
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

    void toggleStep(int i) {
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View::None:
                return;
            case View::Empty:
            case View::ExactlyOnStep:
                steps[start].active ^= true;
                steps[start].skipNext = false;
                steps[start].key = lastKey;
                return;
            case View::ContainsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    steps[i].active = false;
                return;
        }
    }

    void play() {
        play(lastKey);
    }

    void play(int key) {
        lastKey = key;
        if (song->playing && song->armed) {
            auto quantizedStep = song->quantizedStep(resolution);
            steps[quantizedStep % length] = {
                .active = true,
                .skipNext = quantizedStep > song->step,
                .key = key
            };
        }
        restartVoice(key);
    }

    void release() {
        release(lastKey);
    }

    void release(int key) {
        voices->release(keyToNote(key), &entries);
    }

    int keyToNote(int key) {
        return song->keyToNote(useKey, octave, key);
    }

  private:
    int pageStart = 0;
    int length = Song::maxResolution;
    Song* song;
    Samples::Sample* defaultSample;
    Samples::Sample liveSample;
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
            return View::None;
        int countActive = 0;
        int lastActive = 0;
        for (int i = start; i < start + viewLength(); i++) {
            if (steps[i].active) {
                countActive++;
                lastActive = i;
            }
        }
        if (countActive == 0)
            return View::Empty;
        else if (countActive == 1 && lastActive == start)
            return View::ExactlyOnStep;
        else
            return View::ContainsSteps;
    }

    void restartVoice(int key) {
        release(key);
        voices->allocate(
            sampleType,
            keyToNote(key),
            &entries,
            sampleType == Voices::SampleType::File ? defaultSample : &liveSample
        );
    }
};
