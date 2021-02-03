struct Voice {
    struct Output {
        // eventually cast to a float*, so fields should only be floats
        float l;
        float r;
        float controls;
    };

    int volume = 7;
    int pan = 7;
    int filter = 7;
    int resonance;
    int delay;
    int reverb;
    int octave = 3;

    Voice() {
        memory.id = -1;
        memory.stereo = false;
        memory.length = 6*SAMPLE_RATE;
        memory.frames = new float[memory.length];
    }

    void prepare(int note) {
        note = note + octave * 12;
        increment = pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
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
            output.l = sample->left(i);
            output.r = sample->right(i);
            position += increment;
        }
        else if (active && i + 1 < sample->length) {
            output.l = interpolate(position-i, sample->left(i), sample->left(i + 1));
            output.r = interpolate(position-i, sample->right(i), sample->right(i + 1));
            position += increment;
        }
        else {
            active = false;
            output.l = output.r = 0;
        }
        output.controls = volume | pan << 4 | filter << 8 | resonance << 12 | delay << 16 | reverb << 20;
    }

    Sample::Waveform waveform() const {
        return sample->waveform();
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
};
