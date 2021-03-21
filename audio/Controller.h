struct Controller : EventHandler {
    static const int voiceCount = 15;
    std::array<Voice::Output, voiceCount> output;

    Controller(std::filesystem::path root, Destinations* d) : output(), destinations(d), voices(), sequences(), receiveCallbacks(), sendCallbacks(), sendMessages(), destinationMessages() {
        media = std::make_unique<Media>(root);
        for (int i = 0; i < voiceCount; i++)
            voices[i].use(media->get(i));
        sendMessages.reset(8); // max queue size
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["activateVoice"] = &Controller::onActivateVoice;
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["view"] = &Controller::onView;
        sendCallbacks["stepSequence"] = &Controller::onStepSequence;
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["tempo"] = &Controller::onTempo;
        // receive callbacks use lambdas since they are not run on the audio thread, and thus allowed to allocate
        receiveCallbacks["playing"] = [this](){ return playing; };
        receiveCallbacks["armed"] = [this](){ return armed; };
        receiveCallbacks["bars"] = [this](){ return sequences[activeVoice].bars(); };
        receiveCallbacks["viewStart"] = [this](){ return sequences[activeVoice].viewStart(); };
        receiveCallbacks["resolution"] = [this](){ return sequences[activeVoice].resolution; };
        receiveCallbacks["tempo"] = [this](){ return tempo; };
        receiveCallbacks["activeVoice"] = [this](){ return activeVoice; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        receiveCallbacks["naturalNote"] = [this](){ return voices[activeVoice].naturalNote; };
        for (int i = 0; i < voiceCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (int i = 0; i < 4; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return sequences[activeVoice].view(i); };
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
        // step
        bool newStep;
        if (playing) {
            framesSinceLastStep++;
            if (framesSinceLastStep >= stepDuration()) {
                step++;
                framesSinceLastStep = 0;
                newStep = true;
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
            int sequenceNote;
            if (newStep && sequences[v].at(step, sequenceNote))
                voices[v].prepare(sequenceNote);
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
    int step = -1;
    int framesSinceLastStep = 0;
    int transpose = 0;
    int scale = 0;
    int activeVoice = 0;
    Destinations* destinations;
    std::unique_ptr<Media> media;
    std::array<Voice, voiceCount> voices;
    std::array<Sequence, voiceCount> sequences;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, void(Controller::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Controller::*)(int), int>> sendMessages;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<Destinations::Entry*, int>> destinationMessages;

    void onAuditionDown(int value) {
        prepareVoice(value, voices[value].naturalNote);
    }

    void onNoteDown(int value) {
        prepareVoice(activeVoice, keyToNote(value));
    }

    void onActivateVoice(int value) {
        activeVoice = value;
    }

    void onView(int value) {
        switch (value) {
            case 0: return sequences[activeVoice].zoomOut();
            case 1: return sequences[activeVoice].movePage(-1);
            case 2: return sequences[activeVoice].movePage(1);
            case 3: return sequences[activeVoice].zoomIn();
        }
    }

    void onStepSequence(int value) {
        sequences[activeVoice].toggle(value);
    }

    void onPlay(int) {
        playing = !playing;
        step = -1;
        framesSinceLastStep = std::ceil(stepDuration());
    }

    void onArm(int) {
        armed = !armed;
    }

    void onTempo(int value) {
        tempo = std::clamp(tempo + nudge(value), 1, 999);
    }

    void prepareVoice(int v, int note) {
        voices[v].prepare(note);
        if (armed && playing)
            sequences[activeVoice].record(step + (framesSinceLastStep > (stepDuration()/2)), note);
    }

    float stepDuration() {
        return SAMPLE_RATE * 240.f / tempo / Sequence::maxResolution;
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
