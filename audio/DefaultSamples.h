struct DefaultSamples {
    DefaultSamples(std::filesystem::path root) {
        std::string filename;
        int i = 0;
        while (getFilename(root, filename, i++)) {
            unsigned int channels;
            unsigned int sampleRate;
            ma_uint64 length;
            auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &length, NULL);
            assert(frames != NULL);
            assert(channels == 1 || channels == 2);
            assert(sampleRate == SAMPLE_RATE);
            data.push_back({ channels == 1, length, std::unique_ptr<float[]>(frames) });
        }
    }

    Voice::Sample* get(int i) {
        return &data[i];
    }

  private:
    std::vector<Voice::Sample> data;

    bool getFilename(std::filesystem::path root, std::string& filename, int i) {
        auto basename = i < 10 ? "0" + std::to_string(i) : std::to_string(i);
        filename = (root / "default-samples" / (basename + ".wav")).string();
        return std::filesystem::exists(filename);
    }
};
