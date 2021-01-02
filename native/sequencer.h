#include <array>
#include <cmath>

namespace groovebox {
    const int trackCount = 8;
    const int lengthCount = 8;
    const int hitCount = 16;
    const int stepCount = 128;
    const int keyCount = 15;
    const int trackTypeCount = 5;
    const int soundsCount = 15;
    const int octaveCount = 9;
    const int rootCount = 12;
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
        int length;
        int activeSample;
        int sounds;
        int octave = 3;
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    struct Input {
        int play;
        int arm;
        int bpm;
        std::array<int, keyCount> key;
        std::array<int, hitCount> step;
        std::array<int, lengthCount> length;
        std::array<int, trackCount> track;
        std::array<int, trackTypeCount> trackType;
        std::array<int, soundsCount> sounds;
        std::array<int, rootCount> root;
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
        int beat;
        // selections
        int activeKey;
        int activeTrack;
        int activeTrackType;
        int activePage;
        int activeLength;
        int activeSounds;
        int activeOctave;
        std::array<int, hitCount> activeHits;
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
            beat = stepPosition % hitCount;
            if (current.bpm) bpm = current.bpm;

#define TRIGS(prefix, ifTrig)                                \
            auto prefix##Trigs = current.prefix;             \
            for (int i = 0; i < prefix##Trigs.size(); i++) { \
                prefix##Trigs[i] &= !previous.prefix[i];     \
                if (prefix##Trigs[i]) { ifTrig; }            \
            }
            TRIGS(root, root = i)
            TRIGS(scale, scale = i)
            TRIGS(track, activeTrack = i)
            TRIGS(trackType, tracks[activeTrack].type = static_cast<Type>(i))
            TRIGS(length, tracks[activeTrack].length = i)
            TRIGS(sounds, tracks[activeTrack].sounds = i; updateActiveSample())
            TRIGS(octave, tracks[activeTrack].octave = i)
            TRIGS(key, activeKey = i; updateActiveSample(); liveKey())
            TRIGS(step, getBeatStep(i)[activeKey] ^= true)
#undef TRIGS

            activePage = getBeatPage();
            activeLength = tracks[activeTrack].length;
            activeTrackType = tracks[activeTrack].type;
            activeSounds = tracks[activeTrack].sounds;
            activeOctave = tracks[activeTrack].octave;
            for (int s = 0; s < hitCount; s++)
                activeHits[s] = getBeatStep(s)[activeKey];

            for (int t = 0; t < trackCount; t++)
                for (int k = 0; k < keyCount; k++)
                    voiceOut[t][k][Output::position] += voiceIncrements[t][k];
            if (playing && stepPosition != inSteps(framePosition - 1) % stepCount)
                for (int t = 0; t < trackCount; t++)
                    for (int k = 0; k < keyCount; k++)
                        if (getAbsoluteStep(t, stepPosition)[k])
                            useVoice(t, k);

            previous = current;
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60.f * SAMPLE_RATE / bpm) * subdivision * 2);
        }

        void updateActiveSample() {
            if (tracks[activeTrack].type == Type::kit)
                tracks[activeTrack].activeSample = getSample(activeTrack, activeKey);
        }

        int getSample(int t, int key) {
            auto track = tracks[t];
            return track.sounds > 12 ? key*18 + track.sounds + 2 : key + track.sounds*18;
        }

        int getBeatPage() {
            return (stepPosition / hitCount) % (tracks[activeTrack].length + 1);
        }

        bool* getBeatStep(int i) {
            return tracks[activeTrack].steps[i + getBeatPage() * hitCount].data();
        }

        bool* getAbsoluteStep(int t, int i) {
            return tracks[t].steps[i % ((tracks[t].length + 1) * hitCount)].data();
        }

        void liveKey() {
            if (playing && armed) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                getAbsoluteStep(activeTrack, quantizePosition)[activeKey] = true;
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
            voiceOut[t][voiceIndex][Output::sample] = track.type == Type::kit ? getSample(t, key) : track.activeSample;
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
