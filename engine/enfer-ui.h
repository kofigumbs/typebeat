namespace groovebox {
    struct EnferUI: UI {
        const std::array<std::string, 13> enferKits {
            "tr808", "tr909", "dmx", "dnb", "dark",
            "deep", "tech", "modular", "gabber", "bergh",
            "vermona", "commodore", "dmg",
        };

        const std::array<std::string, 18> enferSamples {
            "kick", "kick-up", "kick-down", "tom", "snare",
            "snare-up", "snare-down", "clap", "hat", "hat-open",
            "hat-shut", "cymb", "fx1", "fx2", "fx3",
            "fx4",
            "synth-C2", "synth-C3"
        };

        std::filesystem::path root;
        EnferUI(std::filesystem::path root): root(root) {}

        void addSoundfile(const char* label, const char* filename, Soundfile** sf_zone) override {
            const int fileCount = enferSamples.size() * enferKits.size();
            MA_ASSERT(fileCount <= MAX_SOUNDFILE_PARTS);

            int totalLength = 0;
            float* data[fileCount];
            unsigned int fileChannels[fileCount];
            Soundfile* soundfile = new Soundfile();
            soundfile->fChannels = 2;

            // read each enfer wav file into `data`, tracking metadata in `soundfile` and `fileChannels`
            for (int kit = 0; kit < enferKits.size(); kit++) {
                for (int sample = 0; sample < enferSamples.size(); sample++) {
                    auto i = kit * enferSamples.size() + sample;
                    auto filename = root / "engine" / "Enfer" / "media" / enferKits[kit] / (enferSamples[sample] + ".wav");
                    unsigned int sampleRate;
                    ma_uint64 length;
                    data[i] = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &fileChannels[i], &sampleRate, &length, NULL);
                    MA_ASSERT(data[i] != NULL);
                    soundfile->fSR[i] = sampleRate;
                    soundfile->fOffset[i] = totalLength;
                    soundfile->fLength[i] = length;
                    totalLength += length;
                }
            }

            // fill metadata for remaining soundfile parts
            for (int i = fileCount; i < MAX_SOUNDFILE_PARTS; i++) {
                soundfile->fLength[i] = BUFFER_SIZE;
                soundfile->fSR[i] = SAMPLE_RATE;
                soundfile->fOffset[i] = totalLength;
                totalLength += BUFFER_SIZE;
            }

            // fill actual audio data, now that we know the total buffer size
            soundfile->fBuffers = new float*[soundfile->fChannels];
            for (int channel = 0; channel < soundfile->fChannels; channel++)
                soundfile->fBuffers[channel] = new float[totalLength] {};
            for (int i = 0; i < fileCount; i++) {
                for (int channel = 0; channel < soundfile->fChannels; channel++)
                    if (fileChannels[i] == 1)
                        memcpy(soundfile->fBuffers[channel] + soundfile->fOffset[i], data[i], sizeof(float) * soundfile->fLength[i]);
                    else
                        for (int frame = 0; frame < soundfile->fLength[i]; frame++)
                            soundfile->fBuffers[channel][soundfile->fOffset[i] + frame] = data[i][channel + frame * fileChannels[i]];
                MA_FREE(data[i]);
            }

            *(sf_zone) = soundfile;
        }

        void openTabBox(const char* label) override {}
        void openHorizontalBox(const char* label) override {}
        void openVerticalBox(const char* label) override {}
        void closeBox() override {}
        void declare(float* zone, const char* key, const char* val) override {}
        void addHorizontalBargraph(const char* label, float* zone, float min, float max) override {}
        void addVerticalBargraph(const char* label, float* zone, float min, float max) override {}
        void addCheckButton(const char* label, float* zone) override {}
        void addVerticalSlider(const char* label, float* zone, float init, float min, float max, float step) override {}
        void addHorizontalSlider(const char* label, float* zone, float init, float min, float max, float step) override {}
        void addNumEntry(const char* label, float* zone, float init, float min, float max, float step) override {}
        void addButton(const char* label, float* zone) override {}
    };
}
