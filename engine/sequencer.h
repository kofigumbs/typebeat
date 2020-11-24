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
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    struct Input {
        int play;
        int arm;
        std::array<int, keyCount> key;
        std::array<int, stepCount> step;
        std::array<int, trackCount> track;
        std::array<int, stepCount> trackType;
        std::array<int, stepCount> instrument;
        std::array<int, stepCount> scale;
        std::array<int, stepCount> octave;
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
        int lastKey;
        int framePosition;
        int stepPosition;
        // outputs
        int activeTrack;
        int activeTrackType;
        int activeInstrument;
        int activeOctave;
        // internals
        Input previous;
        std::array<Track, trackCount> tracks;
        std::array<std::array<float, keyCount>, trackCount> voiceIncrements;
        std::array<std::array<std::array<float, Output::count>, keyCount>, trackCount> voiceOut;

        // explicit `init` so that we keep the default, zero-initializing constructor
        void init() {
            for (int t = 0; t < trackCount; t++)
                for (int k = 0; k < keyCount; k++)
                    voiceOut[t][k][Output::sample] = 255;
        }

        void compute(Input current) {
            playing ^= !previous.play && current.play;
            armed ^= !previous.arm && current.arm;
            framePosition = playing ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;

#define TRIGS(prefix, ifTrig)                                \
            auto prefix##Trigs = current.prefix;             \
            for (int i = 0; i < prefix##Trigs.size(); i++) { \
                prefix##Trigs[i] &= !previous.prefix[i];     \
                if (prefix##Trigs[i]) { ifTrig; }            \
            }
            TRIGS(scale, scale = i)
            TRIGS(track, activeTrack = i)
            TRIGS(trackType, tracks[activeTrack].type = i)
            TRIGS(instrument, tracks[activeTrack].instrument = i)
            TRIGS(octave, tracks[activeTrack].octave = i)
            TRIGS(key, lastKey = i; liveKey(i))
            TRIGS(step, tracks[activeTrack].steps[i][lastKey] ^= true)
#undef TRIGS

            activeTrackType = tracks[activeTrack].type;
            activeInstrument = tracks[activeTrack].instrument;
            activeOctave = tracks[activeTrack].octave;

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
