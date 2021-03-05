struct Sequencer : EventHandler {
    static const int voiceCount = 15;
    std::array<Voice::Output, voiceCount> output;

    Sequencer(std::filesystem::path root) : voices(), output(), receiveCallbacks(), sendCallbacks(), sendMessages() {
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
            library[i].stereo = channels == 2;
            library[i].frames = std::unique_ptr<float[]>(frames);
            voices[i].use(library[i]);
        }

        /*
         * handle events
         */
        sendMessages.reset(8); // max queue size
        sendCallbacks["noteDown"] = &Sequencer::onNoteDown;
        sendCallbacks["auditionDown"] = &Sequencer::onAuditionDown;
        sendCallbacks["selectVoice"] = &Sequencer::onSelectVoice;
        sendCallbacks["nudge:eq"] = &Sequencer::onNudgeEq;
        sendCallbacks["nudge:envelope"] = &Sequencer::onNudgeEnvelope;
        sendCallbacks["nudge:effect"] = &Sequencer::onNudgeEffect;
        sendCallbacks["nudge:mix"] = &Sequencer::onNudgeMix;
        // receive callbacks use lambdas since they are not run on the audio thread, and thus allowed to allocate
        receiveCallbacks["activeVoice"] = [this](){ return activeVoice; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        receiveCallbacks["naturalNote"] = [this](){ return voices[activeVoice].naturalNote; };
        for (int i = 0; i < voiceCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (int i = 0; i < voices[activeVoice].eq.size(); i++)
            receiveCallbacks["eq:" + std::to_string(i)] = [this, i](){ return getEq(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].envelope.size(); i++)
            receiveCallbacks["envelope:" + std::to_string(i)] = [this, i](){ return getEnvelope(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].effect.size(); i++)
            receiveCallbacks["effect:" + std::to_string(i)] = [this, i](){ return getEffect(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].mix.size(); i++)
            receiveCallbacks["mix:" + std::to_string(i)] = [this, i](){ return getMix(activeVoice, i); };
    }

    void onSend(std::string name, int value) override {
        auto f = sendCallbacks.find(name);
        if (f != sendCallbacks.end())
            sendMessages.push({ f->second, value });
    }

    int onReceive(std::string name) override {
        auto f = receiveCallbacks.find(name);
        return f != receiveCallbacks.end() ? f->second() : 0;
    }

    void compute(float audio) {
        std::pair<void(Sequencer::*)(int), int> pair;
        while(sendMessages.pop(pair))
            (this->*pair.first)(pair.second);
        for (int v = 0; v < voiceCount; v++)
            voices[v].play(output[v]);
    }

    int getEq(int voice, int id) {
        return voices[voice].eq[id];
    }

    int getEnvelope(int voice, int id) {
        return voices[voice].envelope[id];
    }

    int getEffect(int voice, int id) {
        return voices[voice].effect[id];
    }

    int getMix(int voice, int id) {
        return voices[voice].mix[id];
    }

  private:
    void onAuditionDown(int value) {
        voices[value].prepare(voices[value].naturalNote);
    }

    void onNoteDown(int value) {
        voices[activeVoice].prepare(keyToNote(value));
    }

    void onSelectVoice(int value) {
        activeVoice = value;
        if (!playing)
            onAuditionDown(value);
    }

    void onNudgeEq(int value) {
        nudge(value, voices[activeVoice].eq);
    }

    void onNudgeEnvelope(int value) {
        nudge(value, voices[activeVoice].envelope);
    }

    void onNudgeEffect(int value) {
        nudge(value, voices[activeVoice].effect);
    }

    void onNudgeMix(int value) {
        nudge(value, voices[activeVoice].mix);
    }

    template <size_t T>
    void nudge(int value, std::array<int, T>& destination) {
        int id = std::clamp(value >> 4, 0, (int) destination.size());
        int offset;
        switch (value & 15) {
            case 0: offset = -10; break;
            case 1: offset =  -1; break;
            case 2: offset =   1; break;
            case 3: offset =  10; break;
        }
        destination[id] = std::clamp(destination[id] + offset, 0, 50);
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
    int transpose = 0;
    int scale = 0;
    int activeVoice = 0;
    std::array<Voice::Sample, voiceCount> library;
    std::array<Voice, voiceCount> voices;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, void(Sequencer::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Sequencer::*)(int), int>> sendMessages;

    int keyToNote(int key) {
        return transpose + scaleOffsets[scale][key % 7] + (voices[activeVoice].octave + key/7) * 12;
    }
};
