struct Sequencer {
    static const int voiceCount = 15;
    static const int librarySampleCount = 221;
    EventQueue eventQueue;
    std::array<Voice::Output, voiceCount> output;

    Sequencer(std::filesystem::path root) : eventQueue(), voices(), output() {

        /*
         * load enfer sample library
         */

        const auto media = root / "audio" / "Enfer" / "media";
        const std::array<std::filesystem::path, librarySampleCount> enferPaths {
#define KITS(s) media/"tr808"/s, media/"tr909"/s, media/"dmx"/s, media/"dnb"/s, media/"dark"/s, media/"deep"/s, media/"tech"/s, \
               media/"modular"/s, media/"gabber"/s, media/"bergh"/s, media/"vermona"/s, media/"commodore"/s, media/"dmg"/s
            KITS("kick.wav"), KITS("kick-up.wav"), KITS("kick-down.wav"), KITS("tom.wav"), KITS("snare.wav"), KITS("snare-up.wav"),
            KITS("snare-down.wav"), KITS("clap.wav"), KITS("hat.wav"), KITS("hat-open.wav"), KITS("hat-shut.wav"), KITS("cymb.wav"),
            KITS("fx1.wav"), KITS("fx2.wav"), KITS("fx3.wav"), KITS("fx4.wav"), KITS("synth-C3.wav"),
#undef KITS
        };
        for (int i = 0; i < enferPaths.size(); i++) {
            auto filename = enferPaths[i];
            unsigned int channels;
            unsigned int sampleRate;
            library[i].frames = drwav_open_file_and_read_pcm_frames_f32(filename.string().c_str(), &channels, &sampleRate, &library[i].length, NULL);
            assert(library[i].frames != NULL);
            assert(sampleRate == SAMPLE_RATE);
            assert(channels == 1 || channels == 2);
            library[i].stereo = channels == 2;
            library[i].id = i;
        }
        const std::array<int, voiceCount> initialLayout { 0, 13, 26, 39, 52, 65, 78, 91, 104, 117, 130, 143, 156, 169, 208 };
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(library[initialLayout[i]]);

        /*
         * event handlers
         */

        eventQueue.on("auditionDown", [this](int value) {
            voices[value].prepare(keyToNote(7));
            activeVoice = value;
        });
        eventQueue.on("noteDown", [this](int value) {
            voices[activeVoice].prepare(keyToNote(value));
        });
    }

    void compute(float audio) {
        eventQueue.evaluate();
        for (int v = 0; v < voiceCount; v++)
            voices[v].play(output[v]);
    }

  private:
    const std::array<std::array<int, 7>, 12> scaleOffsets {
        0, 2, 4, 5, 7, 9, 11,
        0, 2, 3, 5, 7, 8, 10,
        0, 2, 3, 5, 7, 9, 10,
        0, 1, 3, 5, 7, 8, 10,
        0, 2, 4, 6, 7, 9, 11,
        0, 2, 4, 5, 7, 9, 10,
        0, 1, 3, 5, 6, 8, 10,
        0, 2, 3, 5, 7, 8, 11,
        0, 2, 4, 5, 7, 8, 11,
        0, 2, 3, 5, 7, 9, 11,
        0, 2, 3, 5, 7, 8, 10,
        0, 2, 4, 5, 7, 8, 10,
    };

    bool playing = false;
    bool armed = false;
    int tempo = 120;
    int root = 0;
    int scale = 0;
    int activeVoice = 0;
    std::array<Voice::Sample, librarySampleCount> library;
    std::array<Voice, voiceCount> voices;

    int keyToNote(int key) {
        return root + scaleOffsets[scale][key % 7] + (key/7 - 1) * 12;
    }
};
