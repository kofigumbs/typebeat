struct SampleFiles : GenericUI {
    SampleFiles(std::filesystem::path dir) {
        assert(Controller::trackCount <= MAX_SOUNDFILE_PARTS);
        soundfile = std::make_unique<Soundfile>();
        soundfile->fChannels = 2;

        // read each enfer wav file into `data`, tracking metadata in `soundfile`
        int totalLength = 0;
        auto data = std::make_unique<float*[]>(Controller::trackCount);
        for (int i = 0; i < Controller::trackCount; i++) {
            auto path = dir / ((i < 10 ? "0" : "") + std::to_string(i) + ".wav");
            unsigned int channels;
            unsigned int sampleRate;
            drwav_uint64 length;
            data[i] = drwav_open_file_and_read_pcm_frames_f32(path.c_str(), &channels, &sampleRate, &length, NULL);
            assert(data[i] != NULL);
            assert(channels == soundfile->fChannels);
            soundfile->fSR[i] = sampleRate;
            soundfile->fLength[i] = length;
            soundfile->fOffset[i] = totalLength;
            totalLength += length;
        }

        // fill metadata for remaining soundfile parts
        for (int i = Controller::trackCount; i < MAX_SOUNDFILE_PARTS; i++) {
            soundfile->fSR[i] = SAMPLE_RATE;
            soundfile->fLength[i] = BUFFER_SIZE;
            soundfile->fOffset[i] = totalLength;
            totalLength += BUFFER_SIZE;
        }

        // fill actual audio data, now that we know the total buffer size
        soundfile->fBuffers = new float*[soundfile->fChannels];
        for (int channel = 0; channel < soundfile->fChannels; channel++)
            soundfile->fBuffers[channel] = new float[totalLength];
        for (int i = 0; i < Controller::trackCount; i++) {
            for (int channel = 0; channel < soundfile->fChannels; channel++)
                for (int frame = 0; frame < soundfile->fLength[i]; frame++)
                    soundfile->fBuffers[channel][soundfile->fOffset[i] + frame] = data[i][channel + frame * soundfile->fChannels];
            MA_FREE(data[i]);
        }
    }

    void addSoundfile(const char* label, const char* filename, Soundfile** sf_zone) override {
        *(sf_zone) = soundfile.get();
    }

  private:
    std::unique_ptr<Soundfile> soundfile;
};
