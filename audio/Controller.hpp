struct Controller : Audio::EventHandler {
    static const int trackCount = 15;
    static const int maxQueueSize = 8;

    Controller(Voices* voices, Samples* samples, Entries entries) : tracks(), song(), receiveCallbacks(), sendCallbacks(), sendMessages(), sendEntries() {
        for (int i = 0; i < Controller::trackCount; i++)
            tracks.push_back(Track(voices, &song, &samples->files[i], entries));
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
        receiveCallbacks["sample:type"] = [this](){ return static_cast<int>(tracks[activeTrack].sampleType); };
        // note mode
        sendCallbacks["noteDown"] = &Controller::onNoteDown;
        sendCallbacks["noteUp"] = &Controller::onNoteUp;
        receiveCallbacks["naturalNote"] = [this](){ return tracks[activeTrack].naturalNote; };
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        // beat mode
        sendCallbacks["play"] = &Controller::onPlay;
        sendCallbacks["arm"] = &Controller::onArm;
        sendCallbacks["tempo"] = &Controller::onTempo;
        sendCallbacks["tempoTaps"] = &Controller::onTempoTaps;
        receiveCallbacks["playing"] = [this](){ return song.playing; };
        receiveCallbacks["armed"] = [this](){ return song.armed; };
        receiveCallbacks["tempo"] = [this](){ return song.tempo; };
        // loop mode
        sendCallbacks["zoomOut"] = &Controller::onZoomOut;
        sendCallbacks["zoomIn"] = &Controller::onZoomIn;
        sendCallbacks["page"] = &Controller::onPage;
        sendCallbacks["bars"] = &Controller::onBars;
        sendCallbacks["stepSequence"] = &Controller::onStepSequence;
        receiveCallbacks["bars"] = [this](){ return tracks[activeTrack].bars(); };
        receiveCallbacks["viewStart"] = [this](){ return tracks[activeTrack].viewStart(); };
        receiveCallbacks["resolution"] = [this](){ return tracks[activeTrack].resolution; };
        for (int i = 0; i < Track::viewsPerPage; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].view(i); };
        // song mode
        sendCallbacks["root"] = &Controller::onRoot;
        sendCallbacks["scale"] = &Controller::onScale;
        receiveCallbacks["root"] = [this](){ return song.root; };
        receiveCallbacks["scale"] = [this](){ return song.scale; };
    }

    void onSend(const std::string& name, int value) override {
        if (sendCallbacks.count(name)) {
            sendMessages.push({ sendCallbacks[name], value });
            return;
        }
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
        song.advance();
        std::pair<void(Controller::*)(int), int> message;
        while(sendMessages.pop(message))
            (this->*message.first)(message.second);
        std::pair<Entries::Control*, int> entry;
        while(sendEntries.pop(entry)) {
            if (entry.first->step == 0)
                entry.first->value = !entry.first->value;
            else if (entry.first->step == 1)
                entry.first->value = std::clamp((float) entry.second, entry.first->min, entry.first->max);
            else
                nudge(entry.first->min, entry.first->max, &entry.first->value, entry.second, entry.first->step);
        }
        for (int i = 0; i < Controller::trackCount; i++)
            tracks[i].run(input);
        tracks.front().voices->run(input, outputL, outputR);
    }

  private:
    int activeTrack = 0;
    Song song;
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

    void onPlay(int) {
        song.togglePlay();
    }

    void onArm(int) {
        song.armed = !song.armed;
    }

    void onTempo(int value) {
        nudge(1, 999, &song.tempo, value, 10);
    }

    void onTempoTaps(int value) {
        song.tempo = value;
    }

    void onZoomOut(int) {
        tracks[activeTrack].zoomOut();
    }

    void onZoomIn(int) {
        tracks[activeTrack].zoomIn();
    }

    void onPage(int diff) {
        tracks[activeTrack].movePage(diff);
    }

    void onBars(int value) {
        tracks[activeTrack].adjustLength(value);
    }

    void onStepSequence(int value) {
        tracks[activeTrack].toggle(value);
    }

    void onScale(int value) {
        song.scale = value;
    }

    void onRoot(int value) {
        nudge(-36, 36, &song.root, value, 12);
    }

    int keyToNote(int key) {
        return song.keyToNote(tracks[activeTrack].octave, key);
    }

    template <typename T>
    void nudge(T low, T high, T* original, int value, int jump) {
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
