struct Controller : EventHandler {
    static const int trackCount = 15;

    Controller(Track track) : tracks(), transport(), receiveCallbacks(), sendCallbacks(), sendMessages() {
        for (int i = 0; i < trackCount; i++) {
            tracks.push_back(track);
            tracks[i].sample = i;
        }
        sendMessages.reset(8); // max queue size
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["activateTrack"] = &Controller::onActivateTrack;
        // sendCallbacks["source"] = &Controller::onSource;
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["view"] = &Controller::onView;
        sendCallbacks["stepSequence"] = &Controller::onStepSequence;
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["tempo"] = &Controller::onTempo;
        // receive callbacks use lambdas since they are not run on the audio thread, and thus allowed to allocate
        receiveCallbacks["playing"] = [this](){ return transport.playing; };
        receiveCallbacks["armed"] = [this](){ return transport.armed; };
        receiveCallbacks["bars"] = [this](){ return tracks[activeTrack].bars(); };
        receiveCallbacks["viewStart"] = [this](){ return tracks[activeTrack].viewStart(); };
        receiveCallbacks["resolution"] = [this](){ return tracks[activeTrack].resolution; };
        receiveCallbacks["tempo"] = [this](){ return transport.tempo; };
        receiveCallbacks["activeTrack"] = [this](){ return activeTrack; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        for (int i = 0; i < trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (int i = 0; i < Track::viewsPerPage; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].view(i); };
    }

    void onSend(std::string name, int value) override {
        if (sendCallbacks.count(name)) {
            sendMessages.push({ sendCallbacks[name], value });
            return;
        }
    }

    int onReceive(std::string name) override {
        return receiveCallbacks.count(name) ? receiveCallbacks[name]() : 0;
    }

    void advance() {
        transport.advance();
        std::pair<void(Controller::*)(int), int> send;
        while(sendMessages.pop(send))
            (this->*send.first)(send.second);
        for (auto& track : tracks)
            track.advance();
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

    int transpose = 0;
    int scale = 0;
    int activeTrack = 0;
    Transport transport;
    std::vector<Track> tracks;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, void(Controller::*)(int)> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<void(Controller::*)(int), int>> sendMessages;

    void onAuditionDown(int value) {
        tracks[value].play();
    }

    void onActivateTrack(int value) {
        activeTrack = value;
    }

    void onNoteDown(int value) {
        tracks[activeTrack].play(keyToNote(value));
    }

    void onView(int value) {
        switch (value) {
            case 0: return tracks[activeTrack].zoomOut();
            case 1: return tracks[activeTrack].movePage(-1);
            case 2: return tracks[activeTrack].movePage(1);
            case 3: return tracks[activeTrack].zoomIn();
        }
    }

    void onStepSequence(int value) {
        tracks[activeTrack].toggle(value);
    }

    void onPlay(int) {
        transport.togglePlay();
    }

    void onArm(int) {
        transport.armed = !transport.armed;
    }

    void onTempo(int value) {
        transport.tempo = std::clamp(transport.tempo + nudge(value), 1, 999);
    }

    int keyToNote(int key) {
        return transpose + scaleOffsets[scale][key % 7] + (tracks[activeTrack].octave + key/7) * 12;
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
