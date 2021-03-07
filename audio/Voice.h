struct Voice {
    struct Output {
        float l;
        float r;
    };

    struct Sample {
        bool mono;
        unsigned long length;
        std::unique_ptr<float[]> frames;
    };

    int octave = 4;
    int naturalNote = 69; // 440 Hz
    std::array<int, 6> eq;
    std::array<int, 4> adsr;
    std::array<int, 5> fx;
    std::array<int, 7> mix;

    Voice() : eq(), adsr(), fx(), mix() {
        eq[3] = 50; // lo pass cut
        adsr[2] = 50; // sustain
        mix[0] = 25; // volume
        mix[1] = 25; // pan
        memory.mono = true;
        memory.length = 6*SAMPLE_RATE;
        memory.frames = std::unique_ptr<float[]>(new float[memory.length]);
    }

    void prepare(int note) {
        active = true;
        position = 0;
        increment = pow(2.0f, note / 12.0f) / pow(2.0f, naturalNote / 12.0f);
    }

    void useMemory() {
        use(&memory);
    }

    void use(const Sample* newSample) {
        if (sample != newSample) {
            active = false;
            sample = newSample;
        }
    }

    void play(Output& output) {
        auto i = int(position);
        if (active && position == i && position < sample->length) {
            output.l = leftChannelAt(i);
            output.r = rightChannelAt(i);
            position += increment;
        }
        else if (active && i + 1 < sample->length) {
            output.l = interpolate(position-i, leftChannelAt(i), leftChannelAt(i + 1));
            output.r = interpolate(position-i, rightChannelAt(i), rightChannelAt(i + 1));
            position += increment;
        }
        else {
            active = false;
            output.l = output.r = 0;
        }
    }

  private:
    bool active = false;
    float position = 0;
    float increment = 0;
    Sample memory;
    const Sample* sample = nullptr;

    float interpolate(float x, float a, float b) {
        return a + x*(b - a);
    }

    float leftChannelAt(int i) {
        return sample->frames[sample->mono ? i : 2*i];
    }

    float rightChannelAt(int i) {
        return sample->frames[sample->mono ? i : 2*i + 1];
    }
};
