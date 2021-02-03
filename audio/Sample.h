struct Sample {
    int id;
    bool stereo;
    ma_uint64 length;
    float* frames;

    float left(int i) const {
        return frames[stereo ? 2*i : i];
    }

    float right(int i) const {
        return frames[stereo ? 2*i + 1 : i];
    }

    /*
     * summary of the sample, for ui
     */
    struct Waveform {
        std::array<float, 500> frames;
    };

    Waveform waveform() const {
        Waveform waveform;
        int samplesPerSummaryGroup = length / waveform.frames.size();
        for (int i = 0; i < waveform.frames.size(); i++) {
            float sum = 0;
            for (int j = 0; j < samplesPerSummaryGroup; j++)
                sum += left(i*waveform.frames.size() + samplesPerSummaryGroup);
            waveform.frames[i] = sum / samplesPerSummaryGroup;
        }
        return waveform;
    }
};


