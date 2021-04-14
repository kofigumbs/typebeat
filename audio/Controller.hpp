struct Controller : EventHandler {
    static const int trackCount = 15;
    static const int maxQueueSize = 8;

    Controller(mydsp_poly* dsp, EntryMap entryMap) : tracks(), transport(), receiveCallbacks(), sendCallbacks(), sendMessages(), sendEntries() {
        for (int i = 0; i < Controller::trackCount; i++)
            tracks.push_back(Track(i, dsp, &transport, entryMap));
        sendMessages.reset(maxQueueSize);
        sendEntries.reset(maxQueueSize);
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["auditionUp"] = &Controller::onAuditionUp;
        sendCallbacks["activateTrack"] = &Controller::onActivateTrack;
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["noteUp"] = &Controller::onNoteUp;
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
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        for (int i = 0; i < Track::viewsPerPage; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].view(i); };
    }

    void onSend(const std::string& name, int value) override {
        if (sendCallbacks.count(name))
            sendMessages.push({ sendCallbacks[name], value });
        auto entry = tracks[activeTrack].entry(name);
        if (entry != nullptr)
            sendEntries.push({ entry, value });
    }

    int onReceive(const std::string& name) override {
        if (receiveCallbacks.count(name))
            return receiveCallbacks[name]();
        else
            return tracks[activeTrack].control(name);
    }

    void advance() {
        transport.advance();
        std::pair<void(Controller::*)(int), int> message;
        while(sendMessages.pop(message))
            (this->*message.first)(message.second);
        std::pair<EntryMap::Entry*, int> entry;
        while(sendEntries.pop(entry))
            nudge(entry.first->min, entry.first->max, &entry.first->value, entry.second);
        for (int i = 0; i < Controller::trackCount; i++)
            tracks[i].advance();
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
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<EntryMap::Entry*, int>> sendEntries;

    void onAuditionDown(int value) {
        tracks[value].play();
    }

    void onAuditionUp(int value) {
        tracks[value].release();
    }

    void onActivateTrack(int value) {
        activeTrack = value;
    }

    void onNoteDown(int value) {
        tracks[activeTrack].play(keyToNote(value));
    }

    void onNoteUp(int value) {
        tracks[activeTrack].release(keyToNote(value));
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
        nudge(1, 999, &transport.tempo, value);
    }

    int keyToNote(int key) {
        return transpose + scaleOffsets[scale][key % 7] + (tracks[activeTrack].octave + key/7) * 12;
    }

    template <typename T>
    void nudge(T low, T high, T* original, int value, int jump = 10) {
        int diff = 0;
        switch (value) {
            case 0: diff = -jump; break;
            case 1: diff = -1;    break;
            case 2: diff = 1;     break;
            case 3: diff = jump;  break;
        }
        *original = std::clamp(*original + diff, low, high);
    }
};
