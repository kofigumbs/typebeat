struct Controller : Audio::EventHandler {
    static const int trackCount = 15;
    static const int maxQueueSize = 8;

    Controller(Voices* voices, Samples* samples, Entries entries) : tracks(), transport(), receiveCallbacks(), sendCallbacks(), sendMessages(), sendEntries() {
        for (int i = 0; i < Controller::trackCount; i++)
            tracks.push_back(Track(voices, &transport, &samples->files[i], entries));
        sendMessages.reset(maxQueueSize);
        sendEntries.reset(maxQueueSize);
        // audition
        sendCallbacks["auditionDown"] = &Controller::onAuditionDown;
        sendCallbacks["auditionUp"] = &Controller::onAuditionUp;
        // track select
        sendCallbacks["activeTrack"] = &Controller::onActiveTrack;
        receiveCallbacks["activeTrack"] = [this](){ return activeTrack; };
        // sound mode
        sendCallbacks["sample:type"] = &Controller::onSampleType;
        receiveCallbacks["sample:type"] = [this](){ return tracks[activeTrack].sampleType; };
        // note mode
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["noteUp"] = &Controller::onNoteUp;
        receiveCallbacks["naturalNote"] = [this](){ return tracks[activeTrack].naturalNote; };
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        // loop mode
        sendCallbacks["view"] = &Controller::onView;
        sendCallbacks["stepSequence"] = &Controller::onStepSequence;
        receiveCallbacks["bars"] = [this](){ return tracks[activeTrack].bars(); };
        receiveCallbacks["viewStart"] = [this](){ return tracks[activeTrack].viewStart(); };
        receiveCallbacks["resolution"] = [this](){ return tracks[activeTrack].resolution; };
        for (int i = 0; i < Track::viewsPerPage; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].view(i); };
        // song mode
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["tempo"] = &Controller::onTempo;
        receiveCallbacks["playing"] = [this](){ return transport.playing; };
        receiveCallbacks["armed"] = [this](){ return transport.armed; };
        receiveCallbacks["tempo"] = [this](){ return transport.tempo; };
        receiveCallbacks["scale"] = [this](){ return scale; };
        receiveCallbacks["transpose"] = [this](){ return transpose; };
    }

    void onSend(const std::string& name, int value) override {
        if (sendCallbacks.count(name))
            sendMessages.push({ sendCallbacks[name], value });
        auto entry = tracks[activeTrack].entry(name);
        if (entry != nullptr)
            sendEntries.push({ entry, value });
    }

    bool onReceive(const std::string& name, int& value) override {
        if (receiveCallbacks.count(name)) {
            value = receiveCallbacks[name]();
            return true;
        }
        return tracks[activeTrack].control(name, value);
    }

    void run(const float input, float& outputL, float& outputR) {
        transport.advance();
        std::pair<void(Controller::*)(int), int> message;
        while(sendMessages.pop(message))
            (this->*message.first)(message.second);
        std::pair<Entries::Control*, int> entry;
        while(sendEntries.pop(entry))
            nudge(entry.first->min, entry.first->max, &entry.first->value, entry.second);
        for (int i = 0; i < Controller::trackCount; i++)
            tracks[i].run(input);
        tracks.front().voices->run(input, outputL, outputR);
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
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<Entries::Control*, int>> sendEntries;

    void onAuditionDown(int value) {
        tracks[value].play();
    }

    void onAuditionUp(int value) {
        tracks[value].release();
    }

    void onActiveTrack(int value) {
        activeTrack = value;
    }

    void onSampleType(int value) {
        tracks[activeTrack].setSampleType(value);
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
