struct Controller : Audio::EventHandler {
    static const int trackCount = 15;
    static const int maxQueueSize = 8;

    Controller(Autosave* a, Voices* v, Samples* samples) : autosave(a), voices(v), song(a), tracks(), receiveCallbacks(), sendCallbacks(), sendQueue() {
        tracks.reserve(Controller::trackCount);
        for (int i = 0; i < Controller::trackCount; i++)
            tracks.emplace_back(i, autosave, voices, samples, &song);
        autosave->load();
        sendQueue.reset(maxQueueSize);
        // audition
        sendCallbacks["auditionDown"] = [this](int value){ tracks[value].play(); };
        sendCallbacks["auditionUp"] = [this](int value){ tracks[value].release(); };
        // track select
        sendCallbacks["activeTrack"] = [this](int value){ activeTrack = value; };
        receiveCallbacks["activeTrack"] = [this](){ return activeTrack; };
        // sound mode
        sendCallbacks["sample:type"] = [this](int value){ tracks[activeTrack].setSampleType(value); };
        sendCallbacks["useKey"] = [this](int){ tracks[activeTrack].useKey ^= 1; };
        receiveCallbacks["sample:type"] = [this](){ return static_cast<int>(tracks[activeTrack].sampleType); };
        receiveCallbacks["useKey"] = [this](){ return tracks[activeTrack].useKey; };
        // note mode
        sendCallbacks["noteDown"] = [this](int value){ tracks[activeTrack].play(value); };
        sendCallbacks["noteUp"] = [this](int value){ tracks[activeTrack].release(value); };
        receiveCallbacks["lastKey"] = [this](){ return tracks[activeTrack].lastKey; };
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["note:" + std::to_string(i)] = [this, i](){ return tracks[activeTrack].keyToNote(i); };
        // beat mode
        sendCallbacks["play"] = [this](int){ song.togglePlay(); };
        sendCallbacks["arm"] = [this](int){ song.armed = !song.armed; };
        sendCallbacks["tempo"] = [this](int value){ nudge(&song.tempo, 1, 999, 10, value); };
        sendCallbacks["tempoTaps"] = [this](int value){ song.tempo = value; };
        receiveCallbacks["playing"] = [this](){ return song.playing; };
        receiveCallbacks["armed"] = [this](){ return song.armed; };
        receiveCallbacks["tempo"] = [this](){ return song.tempo; };
        // loop mode
        sendCallbacks["zoomOut"] = [this](int){ tracks[activeTrack].zoomOut(); };
        sendCallbacks["zoomIn"] = [this](int){ tracks[activeTrack].zoomIn(); };
        sendCallbacks["page"] = [this](int value){ tracks[activeTrack].movePage(value); };
        sendCallbacks["bars"] = [this](int value){ tracks[activeTrack].adjustLength(value); };
        sendCallbacks["sequence"] = [this](int value){ tracks[activeTrack].toggleStep(value); };
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
        // mute mode
        sendCallbacks["mute"] = [this](int value){ tracks[value].mute ^= 1; };
        for (int i = 0; i < Controller::trackCount; i++)
            receiveCallbacks["mute:" + std::to_string(i)] = [this, i](){ return tracks[i].mute; };
    }

    void onSend(const std::string& name, int value) override {
        if (sendCallbacks.count(name)) {
            const auto callback = &sendCallbacks[name];
            sendQueue.push([callback, value]() { (*callback)(value); });
        }
        else {
            Entries::Entry* entry;
            auto found = tracks[activeTrack].entries.find(name, entry) || voices->sendEntries.find(name, entry);
            if (!found)
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
        autosave->save();
    }

    bool onReceive(const std::string& name, int& value) override {
        if (receiveCallbacks.count(name)) {
            value = receiveCallbacks[name]();
            return true;
        }
        Entries::Entry* entry;
        auto found = tracks[activeTrack].entries.find(name, entry) || voices->sendEntries.find(name, entry);
        if (found)
            value = entry->value;
        return found;
    }

    void drop(int i, const void* data) override {
        // TODO
        // call drwav_open_memory_and_read_pcm_frames_f32
        // swap result for Tracks.sampleFile pointer
    }

    void run(const float input, float* output) {
        song.advance();
        std::function<void()> message;
        while(sendQueue.pop(message))
            message();
        for (int i = 0; i < Controller::trackCount; i++)
            tracks[i].run(input);
        voices->run(input, output);
    }

  private:
    int activeTrack = 0;
    Autosave* autosave;
    Voices* voices;
    Song song;
    std::vector<Track> tracks;
    std::unordered_map<std::string, std::function<int()>> receiveCallbacks;
    std::unordered_map<std::string, std::function<void(int)>> sendCallbacks;
    choc::fifo::SingleReaderSingleWriterFIFO<std::function<void()>> sendQueue;

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
