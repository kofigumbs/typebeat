struct Sequencer {
    static const int voiceCount = 15;
    static const int librarySampleCount = 221;
    EventQueue eventQueue;
    std::array<Voice::Output, voiceCount> output;

    Sequencer(std::filesystem::path root) : eventQueue(), voices(), output() {
        // load enfer sample library
        const auto media = root / "audio" / "Enfer" / "media";
        const std::array<std::filesystem::path, librarySampleCount> enferPaths {
#define ALL(s) media/"tr808"/s, media/"tr909"/s, media/"dmx"/s, media/"dnb"/s, media/"dark"/s, media/"deep"/s, media/"tech"/s, \
               media/"modular"/s, media/"gabber"/s, media/"bergh"/s, media/"vermona"/s, media/"commodore"/s, media/"dmg"/s
            ALL("kick.wav"), ALL("kick-up.wav"), ALL("kick-down.wav"), ALL("tom.wav"), ALL("snare.wav"), ALL("snare-up.wav"),
            ALL("snare-down.wav"), ALL("clap.wav"), ALL("hat.wav"), ALL("hat-open.wav"), ALL("hat-shut.wav"), ALL("cymb.wav"),
            ALL("fx1.wav"), ALL("fx2.wav"), ALL("fx3.wav"), ALL("fx4.wav"), ALL("synth-C3.wav"),
#undef ALL
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
        // setup initial sample layout (tr808, with fx1, fx2, and synth)
        const std::array<int, voiceCount> layout { 0, 13, 26, 39, 52, 65, 78, 91, 104, 117, 130, 143, 156, 169, 208 };
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(library[layout[i]]);
        // listen for ui bindings
        eventQueue.on("auditionDown", [this](int value) {
            voices[value].prepare(keyToNote(7));
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
