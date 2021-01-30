struct Sequencer {
    static const int trackCount = 5;
    static const int hitCount = 16;
    static const int stepCount = 128;
    static const int keyCount = 15;

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

    enum Source {
        kit,
        note,
        lineThrough,
        lineRecord,
        linePlay,
    };

    struct Track {
        Source source;
        bool polyphonic = true;
        int length;
        int samplePack;
        int octave = 3;
        int muted;
        int selection;
        int kitSelection;
        int linePosition;
        Library::Sample lineSample;
        Voice::Controls lineControls;
        std::array<Voice::Controls, Library::size> sampleControls;
        std::array<Voice, keyCount> voices;
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    bool playing;
    bool armed;
    int tempo = 120;
    int root;
    int scale;
    int framePosition;
    int stepPosition;
    int activeTrack;
    Library library;
    EventQueue eventQueue;
    std::array<Track, trackCount> tracks;
    std::array<std::array<Voice::Output, keyCount>, trackCount> output;
    std::array<bool, keyCount> receivedKeys;

    Sequencer(std::filesystem::path root) : library(root), eventQueue(), tracks(), output(), receivedKeys() {
        for (int t = 0; t < tracks.size(); t++) {
            tracks[t].lineSample.stereo = false;
            tracks[t].lineSample.length = 6*SAMPLE_RATE;
            tracks[t].lineSample.frames = new float[tracks[t].lineSample.length];
        }
        eventQueue.on("keyDown", [this](int value) {
            receivedKeys[value] = true;
            tracks[activeTrack].selection = value;
            if (tracks[activeTrack].source == Source::kit)
                tracks[activeTrack].kitSelection = value;
            if (playing && armed) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                getAbsoluteStep(activeTrack, quantizePosition)[value] = true;
                if (stepPosition != quantizePosition)
                    receivedKeys[value] = false; // prevent double-trig -- aka live-quantize
            }
        });
        eventQueue.on("track", [this](int value) {
            if (value < 5) // TODO
                activeTrack = value;
        });
        eventQueue.on("samplePack", [this](int value) {
            tracks[activeTrack].samplePack = value;
            for (int v = 0; v < tracks[activeTrack].voices.size(); v++)
                tracks[activeTrack].voices[v].release();
        });
        eventQueue.on("mute", [this](int value) {
            tracks[value].muted ^= true;
        });
    }

    void compute(float audio) {
        framePosition = playing ? framePosition+1 : -1;
        stepPosition = inSteps(framePosition) % stepCount;
        eventQueue.evaluate();
        auto playHit = playing && stepPosition != inSteps(framePosition - 1) % stepCount;
        for (int t = 0; t < tracks.size(); t++)
            for (int v = 0; v < tracks[t].voices.size(); v++)
                setOutput(t, v, audio, t == activeTrack && receivedKeys[v] || playHit && !tracks[t].muted && getAbsoluteStep(t, stepPosition)[v]);
        receivedKeys.fill(false);
    }

    int inSteps(int frames, int subdivision = 1) {
        return floor(frames / (60.f * SAMPLE_RATE / tempo) * subdivision * 2);
    }

    bool* getAbsoluteStep(int t, int i) {
        return tracks[t].steps[i % ((tracks[t].length + 1) * hitCount)].data();
    }

    void setOutput(int t, int key, float audio, bool fresh) {
        switch (tracks[t].source) {
        case Source::kit:
            setOutputSample(t, key, key, keyCount/2, fresh);
            break;
        case Source::note:
            setOutputSample(t, key, tracks[t].kitSelection, key, fresh);
            break;
        case Source::lineThrough:
            setOutputLine(t, key, audio);
            break;
        case Source::lineRecord:
            if (key == 0 && tracks[t].linePosition < tracks[t].lineSample.length)
                tracks[t].lineSample.frames[tracks[t].linePosition++] = audio;
            setOutputLine(t, key, audio);
            break;
        case Source::linePlay:
            setOutputLine(t, key, 0);
            setOutputSampleAudio(t, key, tracks[t].lineSample, key, fresh);
            break;
        }
    }

    void setOutputSample(int t, int key, int sampleKey, int note, bool fresh) {
        auto s = library.sampleId(tracks[t].samplePack, sampleKey);
        setOutputSampleAudio(t, key, library.samples[s], note, fresh);
        tracks[t].sampleControls[s].encode(output[t][key]);
    }

    void setOutputSampleAudio(int t, int key, Library::Sample sample, int note, bool fresh) {
        // mono
        if (!tracks[t].polyphonic && fresh)
            tracks[t].voices[0].prepare(noteIncrement(t, note));
        if (!tracks[t].polyphonic && (fresh || key == 0))
            tracks[t].voices[0].play(sample, output[t][0]);
        if (!tracks[t].polyphonic && key > 0)
            output[t][key].l = output[t][key].r = 0;
        // poly
        if (tracks[t].polyphonic && fresh)
            tracks[t].voices[key].prepare(noteIncrement(t, note));
        if (tracks[t].polyphonic)
            tracks[t].voices[key].play(sample, output[t][key]);
    }

    void setOutputLine(int t, int key, float audio) {
        output[t][key].l = output[t][key].r = key == 0 ? audio : 0;
        tracks[t].lineControls.encode(output[t][key]);
    }

    float noteIncrement(int t, int key) {
        auto note =
            root * (tracks[t].source != Source::kit) +
            scaleOffsets[scale][key % 7] +
            (tracks[t].octave + (key/7 - 1)) * 12;
        return pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
    }
};
