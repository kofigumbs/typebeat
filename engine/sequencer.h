#include <array>
#include <cmath>

namespace groovebox {
    const int trackCount = 8;
    const int stepCount = 16;
    const int keyCount = 15;

    struct Track {
        int type;
        int instrument;
        int octave = 4;
        int nextVoice;
        std::array<std::array<bool, keyCount>, stepCount> steps {};
    };

    struct Input {
        int playDown;
        int armDown;
        std::array<int, trackCount> trackDown {};
        std::array<int, keyCount> keyDown {};
        std::array<int, stepCount> stepDown {};
    };

    enum Output {
        sample,
        position,
        count
    };

    struct Sequencer {
        // song
        float bpm = 120;
        int root;
        int scale;
        // transport
        int playing;
        int armed;
        int activeTrack;
        int lastKey;
        int framePosition;
        int stepPosition;
        // internals
        Input previous;
        std::array<Track, trackCount> tracks {};
        std::array<std::array<float, keyCount>, trackCount> voiceIncrements {};
        std::array<std::array<std::array<float, Output::count>, keyCount>, trackCount> voiceOut {};

        Sequencer() {
            for (int t = 0; t < trackCount; t++)
                for (int k = 0; k < keyCount; k++)
                    voiceOut[t][k][Output::sample] = 255;
        }

        void compute(Input current) {
            playing ^= !previous.playDown && current.playDown;
            armed ^= !previous.armDown && current.armDown;
            framePosition = playing ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;

#define TRIGS(prefix, ifTrig) \
            auto prefix##Trigs = current.prefix##Down; \
            for (int i = 0; i < prefix##Trigs.size(); i++) { \
                prefix##Trigs[i] &= !previous.prefix##Down[i]; \
                if (prefix##Trigs[i]) { ifTrig; } \
            }
            TRIGS(track, activeTrack = i)
            TRIGS(key, lastKey = i; liveKey(i))
            TRIGS(step, tracks[activeTrack].steps[i][lastKey] ^= true)
#undef TRIGS

            for (int t = 0; t < trackCount; t++)
                for (int k = 0; k < keyCount; k++)
                    voiceOut[t][k][Output::position] += voiceIncrements[t][k];
            if (playing && stepPosition != inSteps(framePosition - 1) % stepCount)
                for (int t = 0; t < trackCount; t++)
                    for (int k = 0; k < keyCount; k++)
                        if (tracks[t].steps[stepPosition][k])
                            useVoice(t, k);

            previous = current;
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60 * 44100 / bpm) * subdivision * 2);
        }

        void liveKey(int key) {
            if (playing && armed) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                tracks[activeTrack].steps[quantizePosition][key] = true;
                if (stepPosition != quantizePosition)
                    return; // prevent double-trig -- aka live-quantize
            }
            useVoice(activeTrack, key);
        }

        void useVoice(int track, int key) {
            voiceIncrements[track][key] = 1; // TODO
            voiceOut[track][key][Output::sample] = key; // TODO
            voiceOut[track][key][Output::position] = 0;
        }
    };
}
