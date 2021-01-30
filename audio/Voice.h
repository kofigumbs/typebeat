struct Voice {
    struct Output {
        // eventually cast to a float*, so fields should only be floats
        float l;
        float r;
        float controls;
    };

    struct Controls {
        int volume = 7;
        int pan = 7;
        int filter = 7;
        int resonance;
        int delay;
        int reverb;

        void encode(Output& output) {
            output.controls = volume | pan << 4 | filter << 8 | resonance << 12 | delay << 16 | reverb << 20;
        }
    };

    bool active;
    float position;
    float increment;

    void prepare(float increment_) {
        active = true;
        position = 0;
        increment = increment_;
    }

    void release() {
        active = false;
    }

    void play(Library::Sample sample, Output& output) {
        auto i = int(position);
        if (active && position == i && position < sample.length) {
            output.l = sample.left(i);
            output.r = sample.right(i);
            position += increment;
        }
        else if (active && i + 1 < sample.length) {
            output.l = interpolate(position-i, sample.left(i), sample.left(i + 1));
            output.r = interpolate(position-i, sample.right(i), sample.right(i + 1));
            position += increment;
        }
        else {
            active = false;
            output.l = output.r = 0;
        }
    }

    float interpolate(float x, float a, float b) {
        return a + x*(b - a);
    }
};
