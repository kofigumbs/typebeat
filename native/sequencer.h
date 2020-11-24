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
        int framePosition;
        int stepPosition;
        // selections
        int activeKey;
        int activeTrack;
        int activeTrackType;
        int activeInstrument;
        int activeOctave;
        std::array<int, stepCount> activeHits;
        // internals
        Input previous;
        std::array<Track, trackCount> tracks;
        std::array<std::array<float, keyCount>, trackCount> voiceIncrements;
        std::array<std::array<std::array<float, Output::count>, keyCount>, trackCount> voiceOut;
        const int voiceOutCount = trackCount * keyCount * Output::count;

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
            TRIGS(key, activeKey = i; liveKey(i))
            TRIGS(step, tracks[activeTrack].steps[i][activeKey] ^= true)
#undef TRIGS

            activeTrackType = tracks[activeTrack].type;
            activeInstrument = tracks[activeTrack].instrument;
            activeOctave = tracks[activeTrack].octave;
            for (int s = 0; s < stepCount; s++)
                activeHits[s] = tracks[activeTrack].steps[s][activeKey];

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
            return floor(frames / (60 * SAMPLE_RATE / bpm) * subdivision * 2);
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

        void useVoice(int t, int key) {
            static const std::array<std::array<int, 15>, 12> scaleOffsets {
                -12, -10, -8, -7, -5, -3, -1, 0, 2, 4, 5, 7, 9, 11, 12,
                -12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12,
                -12, -10, -9, -7, -5, -3, -2, 0, 2, 3, 5, 7, 9, 10, 12,
                -12, -11, -9, -7, -5, -4, -2, 0, 1, 3, 5, 7, 8, 10, 12,
                -12, -10, -8, -6, -5, -3, -1, 0, 2, 4, 6, 7, 9, 11, 12,
                -12, -10, -8, -7, -5, -3, -2, 0, 2, 4, 5, 7, 9, 10, 12,
                -12, -11, -9, -7, -6, -4, -2, 0, 1, 3, 5, 6, 8, 10, 12,
                -12, -10, -9, -7, -5, -4, -1, 0, 2, 3, 5, 7, 8, 11, 12,
                -12, -10, -8, -7, -5, -4, -1, 0, 2, 4, 5, 7, 8, 11, 12,
                -12, -10, -9, -7, -5, -3, -1, 0, 2, 3, 5, 7, 9, 11, 12,
                -12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12,
                -12, -10, -8, -7, -5, -4, -2, 0, 2, 4, 5, 7, 8, 10, 12
            };
            auto track = tracks[t];
            voiceOut[t][key][Output::position] = 0;
            switch (tracks[t].type) {
            case 0:
                voiceIncrements[t][key] = 1;
                voiceOut[t][key][Output::sample] = track.instrument == 13 ? key*18 + 15 : key + track.instrument*18;
                break;
            case 1:
                auto note = root + scaleOffsets[scale][key] + track.octave*12;
                auto useHighTargetNote = note > 42;
                auto sampleNote = useHighTargetNote ? 48 : 36;
                voiceIncrements[t][key] = pow(2.0f, note / 12.0f) / pow (2.0f, sampleNote / 12.0f);
                voiceOut[t][key][Output::sample] = track.instrument*18 + 16 + useHighTargetNote;
                break;
            }
        }
    };
}
