struct Transport {
    static const int maxResolution = 64;

    bool playing = false;
    bool armed = false;
    int tempo = 120;
    int step = -1;

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

  private:
    int framesSinceLastStep;

    float stepDuration(int resolution) {
        return SAMPLE_RATE * 240.f / tempo / resolution;
    }
};
