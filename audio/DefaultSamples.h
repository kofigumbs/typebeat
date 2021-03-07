struct DefaultSamples {
    static const int voiceCount = 15;

    DefaultSamples(std::filesystem::path root) {
        for (int i = 0; i < voiceCount; i++) {
            auto filename = root / "default-samples" / (std::to_string(i+1) + ".wav");
            unsigned int channels;
            unsigned int sampleRate;
            auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.string().c_str(), &channels, &sampleRate, &data[i].length, NULL);
            assert(frames != NULL);
            assert(sampleRate == SAMPLE_RATE);
            assert(channels == 1 || channels == 2);
            data[i].stereo = channels == 2;
            data[i].frames = std::unique_ptr<float[]>(frames);
        }
    }

    Voice::Sample* get(int i) {
        return &data[i];
    }

  private:
    std::array<Voice::Sample, voiceCount> data;
};
