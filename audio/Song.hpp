struct Song {
    static const int maxResolution = 64;
    constexpr static const std::array<std::array<int, 7>, 4> scaleOffsets {
        0, 2, 4, 5, 7, 9, 11,
        0, 2, 3, 5, 7, 8, 10,
        0, 2, 3, 5, 7, 8, 11,
        0, 2, 3, 5, 7, 9, 11,
    };

    bool playing = false;
    bool armed = false;
    int tempo = 120;
    int step = -1;
    int root = 0;
    int scale = 0;

    Song(Autosave* autosave) {
        autosave->bind("song.tempo", &tempo);
        autosave->bind("song.root", &root);
        autosave->bind("song.scale", &scale);
    }

    void togglePlay() {
        playing = !playing;
        step = -1;
        framesSinceLastStep = -1;
    }

    void advance() {
        if (playing) {
            framesSinceLastStep++;
            if (framesSinceLastStep >= stepDuration(maxResolution)) {
                step++;
                framesSinceLastStep = 0;
            }
        }
    }

    bool newStep() {
        return playing && framesSinceLastStep == 0;
    }

    int quantizedStep(int resolution) {
        auto scale = maxResolution / resolution;
        auto scaledStep = step / scale * scale;
        auto snapToNext = (step - scaledStep)*stepDuration(maxResolution) + framesSinceLastStep > stepDuration(resolution)/2;
        return scaledStep + scale*snapToNext;
    }

    int keyToNote(bool useKey, int octave, int key) {
        return (octave + key/7) * 12
            + (useKey ? root + scaleOffsets[scale][key % 7] : scaleOffsets[0][key % 7]);
    }

  private:
    int framesSinceLastStep;

    float stepDuration(int resolution) {
        return SAMPLE_RATE * 240.f / tempo / resolution;
    }
};
