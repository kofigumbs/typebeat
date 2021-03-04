struct Voice {
    static const int parameterCount = 7;

    struct Output {
        float l;
        float r;
    };

    struct Sample {
        bool stereo;
        ma_uint64 length;
        std::unique_ptr<float[]> frames;
    };

    int octave = 5;
    std::array<int, parameterCount> parameters;

    Voice() : parameters() {
        memory.stereo = false;
        memory.length = 6*SAMPLE_RATE;
        memory.frames = std::unique_ptr<float[]>(new float[memory.length]);
        parameters.fill(25);
    }

    void prepare(int note) {
        note = note + octave * 12;
        increment = pow(2.0f, note / 12.0f) / pow(2.0f, 69 / 12.0f);
        position = 0;
        active = true;
    }

    void useMemory() {
        use(memory);
    }

    void use(const Sample& newSample) {
        if (sample != &newSample) {
            active = false;
            sample = &newSample;
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
        return sample->frames[sample->stereo ? 2*i : i];
    }

    float rightChannelAt(int i) {
        return sample->frames[sample->stereo ? 2*i + 1 : i];
    }
};

const int Voice::parameterCount;
