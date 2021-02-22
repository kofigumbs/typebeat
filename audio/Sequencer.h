struct Sequencer : EventQueue {
    static const int voiceCount = 15;
    static const int librarySampleCount = 221;
    std::array<Voice::Output, voiceCount> output;

    Sequencer(std::filesystem::path root) : eventHandlers(), events(), voices(), output() {

        /*
         * load enfer sample library
         */

        const auto media = root / "vendor" / "Enfer" / "media";
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
            auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.string().c_str(), &channels, &sampleRate, &library[i].length, NULL);
            assert(frames != NULL);
            assert(sampleRate == SAMPLE_RATE);
            assert(channels == 1 || channels == 2);
            library[i].id = i;
            library[i].stereo = channels == 2;
            library[i].frames = std::unique_ptr<float[]>(frames);
        }
        const std::array<int, voiceCount> initialLayout { 0, 13, 26, 39, 52, 65, 78, 91, 104, 117, 130, 143, 156, 169, 208 };
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(library[initialLayout[i]]);

        /*
         * event handlers
         */

        events.reset(8); // max queue size
        eventHandlers["auditionDown"] = &Sequencer::auditionDown;
        eventHandlers["noteDown"] = &Sequencer::noteDown;
    }

    void push(std::string method, int value) override {
        auto f = eventHandlers.find(method);
        if (f != eventHandlers.end())
            events.push({ f->second, value });
    }

    void compute(float audio) {
        std::pair<void(Sequencer::*)(int), int> pair;
        while(events.pop(pair))
            (this->*pair.first)(pair.second);
        for (int v = 0; v < voiceCount; v++)
            voices[v].play(output[v]);
    }

    void auditionDown(int value) {
        voices[value].prepare(keyToNote(7));
        activeVoice = value;
    }

    void noteDown(int value) {
        voices[activeVoice].prepare(keyToNote(value));
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
    std::unordered_map<std::string, void(Sequencer::*)(int)> eventHandlers;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Sequencer::*)(int), int>> events;

    int keyToNote(int key) {
        return root + scaleOffsets[scale][key % 7] + (key/7 - 1) * 12;
    }
};
