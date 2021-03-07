struct Controller : EventHandler {
    static const int voiceCount = 15;
    std::array<Voice::Output, voiceCount> output;

    Controller(DefaultSamples* defaultSamples) : voices(), output(), receiveCallbacks(), sendCallbacks(), sendMessages() {
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(defaultSamples->get(i));
        sendMessages.reset(8); // max queue size
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["selectVoice"] = &Controller::onSelectVoice;
        sendCallbacks["nudge:eq"] = &Controller::onNudgeEq;
        sendCallbacks["nudge:adsr"] = &Controller::onNudgeAdsr;
        sendCallbacks["nudge:fx"] = &Controller::onNudgeFx;
        sendCallbacks["nudge:mix"] = &Controller::onNudgeMix;
        // receive callbacks use lambdas since they are not run on the audio thread, and thus allowed to allocate
        receiveCallbacks["playing"] = [this](){ return playing; };
        receiveCallbacks["armed"] = [this](){ return armed; };
        receiveCallbacks["activeVoice"] = [this](){ return activeVoice; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        receiveCallbacks["naturalNote"] = [this](){ return voices[activeVoice].naturalNote; };
        for (int i = 0; i < voiceCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (int i = 0; i < voices[activeVoice].eq.size(); i++)
            receiveCallbacks["eq:" + std::to_string(i)] = [this, i](){ return getEq(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].adsr.size(); i++)
            receiveCallbacks["adsr:" + std::to_string(i)] = [this, i](){ return getAdsr(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].fx.size(); i++)
            receiveCallbacks["fx:" + std::to_string(i)] = [this, i](){ return getFx(activeVoice, i); };
        for (int i = 0; i < voices[activeVoice].mix.size(); i++)
            receiveCallbacks["mix:" + std::to_string(i)] = [this, i](){ return getMix(activeVoice, i); };
    }

    int getEq(int voice, int id) {
        return voices[voice].eq[id];
    }

    int getAdsr(int voice, int id) {
        return voices[voice].adsr[id];
    }

    int getFx(int voice, int id) {
        return voices[voice].fx[id];
    }

    int getMix(int voice, int id) {
        return voices[voice].mix[id];
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
        std::pair<void(Controller::*)(int), int> pair;
        while(sendMessages.pop(pair))
            (this->*pair.first)(pair.second);
        for (int v = 0; v < voiceCount; v++)
            voices[v].play(step, armed, output[v]);
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
    int transpose = 0;
    int scale = 0;
    int activeVoice = 0;
    std::array<Voice, voiceCount> voices;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, void(Controller::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Controller::*)(int), int>> sendMessages;

    void play(int) {
        playing = !playing;
        framePosition = -1;
    }

    void arm(int) {
        armed = !armed;
    }

    void onNoteDown(int value) {
        voices[activeVoice].prepare(keyToNote(value));
    }

    void onAuditionDown(int value) {
        voices[value].prepare(voices[value].naturalNote);
    }

    void onSelectVoice(int value) {
        activeVoice = value;
        if (!playing)
            onAuditionDown(value);
    }

    void onNudgeEq(int value) {
        nudge(value, voices[activeVoice].eq);
    }

    void onNudgeAdsr(int value) {
        nudge(value, voices[activeVoice].adsr);
    }

    void onNudgeFx(int value) {
        nudge(value, voices[activeVoice].fx);
    }

    void onNudgeMix(int value) {
        nudge(value, voices[activeVoice].mix);
    }

    template <size_t T>
    void nudge(int value, std::array<int, T>& destination) {
        int id = std::clamp(value >> 4, 0, (int) destination.size());
        int offset;
        switch (value & 0xf) {
            case 0: offset = -10; break;
            case 1: offset =  -1; break;
            case 2: offset =   1; break;
            case 3: offset =  10; break;
        }
        destination[id] = std::clamp(destination[id] + offset, 0, 50);
    }

    int keyToNote(int key) {
        return transpose + scaleOffsets[scale][key % 7] + (voices[activeVoice].octave + key/7) * 12;
    }
};
