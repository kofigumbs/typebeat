struct Controller : Audio::EventHandler {
    static const int trackCount = 15;
    static const int maxQueueSize = 8;

    Controller(Voices* voices, Samples* samples, Entries entries) : tracks(), song(), receiveCallbacks(), sendCallbacks(), sendQueue() {
        for (int i = 0; i < Controller::trackCount; i++)
            tracks.push_back(Track(voices, &song, &samples->files[i], entries));
        sendQueue.reset(maxQueueSize);
        // audition
        sendCallbacks["auditionDown"] = [this](int value){ tracks[value].play(); };
        sendCallbacks["auditionUp"] = [this](int value){ tracks[value].release(); };
        // track select
        sendCallbacks["activeTrack"] = [this](int value){ activeTrack = value; };
        receiveCallbacks["activeTrack"] = [this](){ return activeTrack; };
        // sound mode
        sendCallbacks["sample:type"] = [this](int value){ tracks[activeTrack].setSampleType(value); };
        receiveCallbacks["sample:type"] = [this](){ return static_cast<int>(tracks[activeTrack].sampleType); };
        // note mode
        sendCallbacks["noteDown"] = [this](int value){ tracks[activeTrack].play(keyToNote(value)); };
        sendCallbacks["noteUp"] = [this](int value){ tracks[activeTrack].release(keyToNote(value)); };
        receiveCallbacks["naturalNote"] = [this](){ return tracks[activeTrack].naturalNote; };
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return keyToNote(i); };
        // beat mode
        sendCallbacks["play"] = [this](int value){ song.togglePlay(); };
        sendCallbacks["arm"] = [this](int value){ song.armed = !song.armed; };
        sendCallbacks["tempo"] = [this](int value){ nudge(&song.tempo, 1, 999, 10, value); };
        sendCallbacks["tempoTaps"] = [this](int value){ song.tempo = value; };
        receiveCallbacks["playing"] = [this](){ return song.playing; };
        receiveCallbacks["armed"] = [this](){ return song.armed; };
        receiveCallbacks["tempo"] = [this](){ return song.tempo; };
        // loop mode
        sendCallbacks["zoomOut"] = [this](int value){ tracks[activeTrack].zoomOut(); };
        sendCallbacks["zoomIn"] = [this](int value){ tracks[activeTrack].zoomIn(); };
        sendCallbacks["page"] = [this](int value){ tracks[activeTrack].movePage(value); };
        sendCallbacks["bars"] = [this](int value){ tracks[activeTrack].adjustLength(value); };
        sendCallbacks["stepSequence"] = [this](int value){ tracks[activeTrack].toggle(value); };
        receiveCallbacks["bars"] = [this](){ return tracks[activeTrack].bars(); };
        receiveCallbacks["viewStart"] = [this](){ return tracks[activeTrack].viewStart(); };
        receiveCallbacks["resolution"] = [this](){ return tracks[activeTrack].resolution; };
        for (int i = 0; i < Track::viewsPerPage; i++)
            receiveCallbacks["view:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].view(i); };
        // song mode
        sendCallbacks["root"] = [this](int value){ nudge(&song.root, -12, 12, 7, value); };
        sendCallbacks["scale"] = [this](int value){ song.scale = value; };
        receiveCallbacks["root"] = [this](){ return song.root; };
        receiveCallbacks["scale"] = [this](){ return song.scale; };
    }

    void onSend(const std::string& name, int value) override {
        if (sendCallbacks.count(name)) {
            const auto callback = &sendCallbacks[name];
            sendQueue.push([callback, value]() { (*callback)(value); });
            return;
        }
        auto entry = tracks[activeTrack].entry(name);
        if (entry == nullptr)
            return;
        sendQueue.push([entry, value]() {
            if (entry->step == 0)
                entry->value = !entry->value;
            else if (entry->step == 1)
                entry->value = std::clamp((float) value, entry->min, entry->max);
            else
                nudge(&entry->value, entry->min, entry->max, entry->step, value);
        });
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
        std::function<void()> message;
        while(sendQueue.pop(message))
            message();
        for (int i = 0; i < Controller::trackCount; i++)
            tracks[i].run(input);
        tracks.front().voices->run(input, outputL, outputR);
    }

  private:
    int activeTrack = 0;
    Song song;
    std::vector<Track> tracks;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, std::function<void(int)>> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::function<void()>> sendQueue;

    int keyToNote(int key) {
        return song.keyToNote(tracks[activeTrack].octave, key);
    }

    template <typename T>
    static void nudge(T* original, T low, T high, int jump, int value) {
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
