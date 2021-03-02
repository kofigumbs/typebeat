struct Sequencer : EventHandler {
    static const int voiceCount = 15;
    std::array<Voice::Output, voiceCount> output;

    Sequencer(std::filesystem::path root) : voices(), output(), receivePointers(), sendCallbacks(), sendMessages() {
        /*
         * load enfer sample library
         */
        for (int i = 0; i < voiceCount; i++) {
            auto filename = root / "default-samples" / (std::to_string(i+1) + ".wav");
            unsigned int channels;
            unsigned int sampleRate;
            auto frames = drwav_open_file_and_read_pcm_frames_f32(filename.string().c_str(), &channels, &sampleRate, &library[i].length, NULL);
            assert(frames != NULL);
            assert(sampleRate == SAMPLE_RATE);
            assert(channels == 1 || channels == 2);
            library[i].id = i;
            library[i].stereo = channels == 2;
            library[i].frames = std::unique_ptr<float[]>(frames);
            voices[i].use(library[i]);
        }

        /*
         * handle events
         */
        sendMessages.reset(8); // max queue size
        sendCallbacks["noteDown"] = &Sequencer::noteDown;
        sendCallbacks["selectVoice"] = &Sequencer::selectVoice;
        sendCallbacks["auditionDown"] = &Sequencer::auditionDown;
        receivePointers["selectedVoice"] = &selectedVoice;
    }

    void onSend(std::string name, int value) override {
        auto f = sendCallbacks.find(name);
        if (f != sendCallbacks.end())
            sendMessages.push({ f->second, value });
    }

    int onReceive(std::string name) override {
        auto p = receivePointers.find(name);
        return p != receivePointers.end() ? *(p->second) : 0;
    }

    void compute(float audio) {
        std::pair<void(Sequencer::*)(int), int> pair;
        while(sendMessages.pop(pair))
            (this->*pair.first)(pair.second);
        for (int v = 0; v < voiceCount; v++)
            voices[v].play(output[v]);
    }

  private:
    void selectVoice(int value) {
        selectedVoice = value;
    }

    void noteDown(int value) {
        voices[selectedVoice].prepare(keyToNote(value));
    }

    void auditionDown(int value) {
        voices[value].prepare(9);
    }

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
    int selectedVoice = 0;
    std::array<Voice::Sample, voiceCount> library;
    std::array<Voice, voiceCount> voices;
    std::unordered_map<std::string, int*> receivePointers;
    std::unordered_map<std::string, void(Sequencer::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Sequencer::*)(int), int>> sendMessages;

    int keyToNote(int key) {
        return root + scaleOffsets[scale][key % 7] + (key/7 - 1) * 12;
    }
};
