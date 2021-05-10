struct Track {
    static const int keyCount = 15;
    static const int viewsPerPage = 4;
    static const int maxLiveRecordLength = 60*SAMPLE_RATE;

    enum class View {
        None,
        Empty,
        ExactlyOnStep,
        ContainsSteps,
    };

    struct Change {
        bool active;
        bool skipNext;
    };

    struct Step {
        Change keyDown[keyCount];
    };

    bool mute = false;
    bool useKey = true;
    int lastKey = 12; // 440Hz (concert pitch A) in C Major
    int resolution = 4;
    int octave = 4;
    Voices::SampleType sampleType = Voices::SampleType::File;
    Entries entries;

    Track(int id, Autosave* autosave, Voices* v, Samples* samples, Song* s) : voices(v), song(s), defaultSample(&samples->data[id]), entries(v->trackEntries()), sequence() {
        liveSample.frames.reset(new float[maxLiveRecordLength]);
        auto prefix = "tracks[" + std::to_string(id) + "].";
        entries.bind(prefix, autosave);
        autosave->bind(prefix + "mute", new Autosave::Number(mute));
        autosave->bind(prefix + "useKey", new Autosave::Number(useKey));
        autosave->bind(prefix + "lastKey", new Autosave::Number(lastKey));
        autosave->bind(prefix + "resolution", new Autosave::Number(resolution));
        autosave->bind(prefix + "octave", new Autosave::Number(octave));
        autosave->bind(prefix + "length", new Autosave::Number(length));
        autosave->bind(prefix + "sampleType", new Autosave::Number(sampleType));
        for (int key = 0; key < keyCount; key++) {
            autosave->bind(
                prefix + "sequence.keyDown[" + std::to_string(key) + "]",
                new Autosave::Array(
                    sequence,
                    [key](auto& step) -> auto& { return step.keyDown[key].active; },
                    [key](auto& step) { return new Autosave::Number(step.keyDown[key].active); }
                )
            );
        }
    }

    void run(const float input) {
        if (sampleType == Voices::SampleType::LiveRecord && liveSample.length < maxLiveRecordLength)
            liveSample.frames[liveSample.length++] = input;
        if (song->newStep()) {
            auto& step = sequence[song->step % length];
            for (int key = 0; key < keyCount; key++) {
                if(replay(step.keyDown[key]) && !mute)
                    restartVoice(key);
            }
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
        length = std::clamp(length + diff*Song::maxResolution, min, (int) sequence.size());
    }

    void toggleStep(int i) {
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View::None:
                return;
            case View::Empty:
            case View::ExactlyOnStep:
                sequence[start].keyDown[lastKey].active ^= true;
                sequence[start].keyDown[lastKey].skipNext = false;
                return;
            case View::ContainsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    sequence[i].keyDown[lastKey].active = false;
                return;
        }
    }

    void play() {
        play(lastKey);
    }

    void play(int key) {
        lastKey = key;
        if (song->playing && song->armed)
            record([this, key](auto& step) -> auto& { return step.keyDown[key]; });
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
    Voices* voices;
    Song* song;
    Samples::Sample* defaultSample;
    Samples::Sample liveSample;
    std::array<Step, Song::maxResolution*16*8> sequence;

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
            if (sequence[i].keyDown[lastKey].active) {
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


    template <typename F>
    void record(F&& f) {
        auto quantizedStep = song->quantizedStep(resolution);
        auto& change = f(sequence[quantizedStep % length]);
        change.active = true;
        change.skipNext = quantizedStep > song->step;
    }

    bool replay(Change& change) {
        if (change.skipNext) {
            change.skipNext = false;
            return false;
        }
        return change.active;
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
