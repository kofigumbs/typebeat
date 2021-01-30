struct Library {
    struct Sample {
        bool stereo;
        ma_uint64 length;
        float* frames;

        float left(int i) {
            return frames[stereo ? 2*i : i];
        }

        float right(int i) {
            return frames[stereo ? 2*i + 1 : i];
        }
    };

    static const int size = 13*17;
    std::array<Sample, Library::size> samples;

    Library(std::filesystem::path root) : samples() {
        const std::array<std::string, 13> enferKits {
            "tr808", "tr909", "dmx", "dnb", "dark", "deep", "tech",
            "modular", "gabber", "bergh", "vermona", "commodore", "dmg",
        };
        const std::array<std::string, 17> enferSamples {
            "kick", "kick-up", "kick-down", "tom", "snare", "snare-up", "snare-down", "clap",
            "hat", "hat-open", "hat-shut", "cymb", "fx1", "fx2", "fx3", "fx4", "synth-C3"
        };
        for (int kit = 0; kit < enferKits.size(); kit++) {
            for (int sample = 0; sample < enferSamples.size(); sample++) {
                auto i = kit * enferSamples.size() + sample;
                auto filename = root / "audio" / "Enfer" / "media" / enferKits[kit] / (enferSamples[sample] + ".wav");
                unsigned int channels;
                unsigned int sampleRate;
                samples[i].frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &samples[i].length, NULL);
                assert(samples[i].frames != NULL);
                assert(sampleRate == SAMPLE_RATE);
                assert(channels == 1 || channels == 2);
                samples[i].stereo = channels == 2;
            }
        }
    }

    int sampleId(int pack, int key) {
        if (pack == 13) // fx4
            return std::min(int(samples.size() - 1), (key + 1)*17 - 2);
        else if (pack == 14) // synths
            return std::min(int(samples.size() - 1), (key + 1)*17 - 1);
        else
            return key + pack*17;
    }
};
