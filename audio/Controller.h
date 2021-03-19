struct Controller : EventHandler {
    static const int voiceCount = 15;
    std::array<Voice::Output, voiceCount> output;

    Controller(std::filesystem::path root, Destinations* d) : output(), destinations(d), voices(), receiveCallbacks(), sendCallbacks(), sendMessages(), destinationMessages() {
        media = std::make_unique<Media>(root);
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(media->get(i));
        sendMessages.reset(8); // max queue size
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["activateVoice"] = &Controller::onActivateVoice;
        sendCallbacks["tempo"] = &Controller::onTempo;
        // receive callbacks use lambdas since they are not run on the audio thread, and thus allowed to allocate
        receiveCallbacks["playing"] = [this](){ return playing; };
        receiveCallbacks["armed"] = [this](){ return armed; };
        receiveCallbacks["beat"] = [this](){ return beat; };
        receiveCallbacks["tempo"] = [this](){ return tempo; };
        receiveCallbacks["activeVoice"] = [this](){ return activeVoice; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        receiveCallbacks["naturalNote"] = [this](){ return voices[activeVoice].naturalNote; };
        for (int i = 0; i < voiceCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (const auto& name : destinations->names)
            receiveCallbacks[name] = [this, name](){ return destinations->get(activeVoice, name)->read(); };
    }

    void onSend(std::string name, int value) override {
        if (sendCallbacks.count(name)) {
            sendMessages.push({ sendCallbacks[name], value });
            return;
        }
        auto destination = destinations->get(activeVoice, name);
        if (destination != nullptr)
            destinationMessages.push({ destination, value });
    }

    int onReceive(std::string name) override {
        return receiveCallbacks.count(name) ? receiveCallbacks[name]() : 0;
    }

    void render(float audio) {
        // beat
        bool newBeat;
        if (playing) {
            framesSinceLastBeat++;
            if (framesSinceLastBeat >= beatDuration()) {
                beat++;
                framesSinceLastBeat = 0;
                newBeat = true;
            }
        }
        // handle send messages
        std::pair<void(Controller::*)(int), int> send;
        while(sendMessages.pop(send))
            (this->*send.first)(send.second);
        // handle destination messages
        std::pair<Destinations::Entry*, int> destination;
        while(destinationMessages.pop(destination))
            destination.first->write(destination.first->read() + nudge(destination.second));
        // render voices
        for (int v = 0; v < voiceCount; v++) {
            auto step = voices[v].sequence[beat % voices[v].sequence.size()];
            if (newBeat && step.active)
                voices[v].prepare(step.note);
            voices[v].render(output[v]);
        }
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
    int beat = -1;
    int framesSinceLastBeat = 0;
    int transpose = 0;
    int scale = 0;
    int activeVoice = 0;
    Destinations* destinations;
    std::unique_ptr<Media> media;
    std::array<Voice, voiceCount> voices;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, void(Controller::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Controller::*)(int), int>> sendMessages;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<Destinations::Entry*, int>> destinationMessages;

    void onPlay(int) {
        playing = !playing;
        beat = -1;
        framesSinceLastBeat = std::ceil(beatDuration());
    }

    void onArm(int) {
        armed = !armed;
    }

    void onNoteDown(int value) {
        prepareVoice(activeVoice, keyToNote(value));
    }

    void onAuditionDown(int value) {
        prepareVoice(value, voices[value].naturalNote);
    }

    void onActivateVoice(int value) {
        activeVoice = value;
    }

    void onTempo(int value) {
        tempo = std::clamp(tempo + nudge(value), 1, 999);
    }

    void prepareVoice(int v, int note) {
        voices[v].prepare(note);
        if (armed && playing) {
            int quantizedBeat = beat + (framesSinceLastBeat > (beatDuration()/2));
            Voice::Step& step = voices[v].sequence[quantizedBeat % voices[v].sequence.size()];
            step.active = true;
            step.note = note;
        }
    }

    float beatDuration() {
        return SAMPLE_RATE * 60.f / tempo;
    }

    int keyToNote(int key) {
        return transpose + scaleOffsets[scale][key % 7] + (voices[activeVoice].octave + key/7) * 12;
    }

    int nudge(int value, int jump = 10) {
        switch (value) {
            case 0:  return -jump;
            case 1:  return -1;
            case 2:  return 1;
            case 3:  return jump;
            default: return 0;
        }
    }
};
