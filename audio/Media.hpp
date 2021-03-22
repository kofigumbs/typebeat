struct Media {
    Media(std::filesystem::path root) {
        std::string filename;
        int i = 0;
        while (hasDefault(root, filename, i++))
            load(filename);
        load(wavFile(root, "metronome"));
    }

    Voice::Sample* get(int i) {
        return &data[i];
    }

  private:
    std::vector<Voice::Sample> data;

    bool hasDefault(std::filesystem::path root, std::string& filename, int i) {
        filename = wavFile(root, std::string("default-") + (i < 10 ? "0" : "") + std::to_string(i));
        return std::filesystem::exists(filename);
    }

    void load(std::string filename) {
        unsigned int channels;
        unsigned int sampleRate;
        ma_uint64 length;
        auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &length, NULL);
        assert(frames != NULL);
        assert(channels == 1 || channels == 2);
        assert(sampleRate == SAMPLE_RATE);
        data.push_back({ channels == 1, length, std::unique_ptr<float[]>(frames) });
    }

    std::string wavFile(std::filesystem::path root, std::string basename) {
        return (root / "audio" / "media" / (basename + ".wav")).string();
    }
};
