#include <array>
#include <cmath>

namespace groovebox {
    const int trackCount = 8;
    const int stepCount = 16;
    const int keyCount = 15;
    const int trackTypeCount = 5;
    const int instrumentCount = 15;
    const int octaveCount = 9;
    const int scaleCount = 12;
    const int outputCount = 2;

    enum Type {
        kit,
        mono,
        poly,
        arp,
        chord
    };

    struct Track {
        Type type;
        int activeSample;
        int instrument;
        int octave = 3;
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    struct Input {
        int play;
        int arm;
        int bpm;
        std::array<int, keyCount> key;
        std::array<int, stepCount> step;
        std::array<int, trackCount> track;
        std::array<int, trackTypeCount> trackType;
        std::array<int, instrumentCount> instrument;
        std::array<int, octaveCount> octave;
        std::array<int, scaleCount> scale;
    };

    enum Output {
        sample,
        position
    };

    struct Sequencer {
        // song
        int root;
        int scale;
        int bpm = 120;
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
        std::array<std::array<std::array<float, outputCount>, keyCount>, trackCount> voiceOut;
        const int voiceOutCount = trackCount * keyCount * outputCount;

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
            if (current.bpm) bpm = current.bpm;

#define TRIGS(prefix, ifTrig)                                \
            auto prefix##Trigs = current.prefix;             \
            for (int i = 0; i < prefix##Trigs.size(); i++) { \
                prefix##Trigs[i] &= !previous.prefix[i];     \
                if (prefix##Trigs[i]) { ifTrig; }            \
            }
            TRIGS(scale, scale = i)
            TRIGS(track, activeTrack = i)
            TRIGS(trackType, tracks[activeTrack].type = static_cast<Type>(i))
            TRIGS(instrument, tracks[activeTrack].instrument = i; updateActiveSample())
            TRIGS(octave, tracks[activeTrack].octave = i)
            TRIGS(key, activeKey = i; updateActiveSample(); liveKey())
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
            return floor(frames / (60.f * SAMPLE_RATE / bpm) * subdivision * 2);
        }

        void updateActiveSample() {
            auto track = tracks.data() + activeTrack;
            if (track->type == Type::kit)
                track->activeSample = track->instrument > 12 ? activeKey*18 + track->instrument + 2 : activeKey + track->instrument*18;
        }

        void liveKey() {
            if (playing && armed) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                tracks[activeTrack].steps[quantizePosition][activeKey] = true;
                if (stepPosition != quantizePosition)
                    return; // prevent double-trig -- aka live-quantize
            }
            useVoice(activeTrack, activeKey);
        }

        void useVoice(int t, int key) {
            static const std::array<std::array<int, keyCount>, scaleCount> scaleOffsets {
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
            auto voiceIndex = track.type == Type::mono ? 0 : key;
            voiceOut[t][voiceIndex][Output::position] = 0;
            voiceOut[t][voiceIndex][Output::sample] = track.activeSample;
            switch (tracks[t].type) {
            case Type::kit:
                voiceIncrements[t][voiceIndex] = 1;
                break;
            default:
                auto note = root + scaleOffsets[scale][key] + track.octave*12;
                voiceIncrements[t][voiceIndex] = pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
                break;
            }
        }
    };
}
