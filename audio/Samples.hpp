struct Samples {
    struct Sample {
        bool stereo;
        ma_uint64 length;
        std::unique_ptr<float[]> frames;
    };

    std::vector<Sample> data;

    Samples(std::filesystem::path directory) {
        std::string filename;
        int i = 0;
        while (hasSample(directory, filename, i++))
            read(filename);
    }

  private:
    bool hasSample(std::filesystem::path directory, std::string& filename, int i) {
        filename = directory / (twoDigit(i) + ".wav");
        return std::filesystem::exists(filename);
    }

    void read(std::string filename) {
        unsigned int channels;
        unsigned int sampleRate;
        ma_uint64 length;
        auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &length, NULL);
        assert(frames != NULL);
        assert(channels == 1 || channels == 2);
        assert(sampleRate == SAMPLE_RATE);
        data.push_back({
            .stereo = channels == 2,
            .length = length,
            .frames = std::unique_ptr<float[]>(frames)
        });
    }
};
