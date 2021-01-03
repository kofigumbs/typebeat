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
        int key;
        int length;
        int track;
        int trackType;
        int sounds;
        int root;
        int octave;
        int scale;
        std::array<int, hitCount> steps;
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
        // active (for ui)
        int activeKey;
        int activeTrack;
        int activeTrackType;
        int activePage;
        int activeLength;
        int activeSounds;
        int activeOctave;
        std::array<int, hitCount> activeSteps;
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
#define received(x) current.x && current.x != previous.x
            // trig
            playing ^= received(play);
            armed ^= received(arm);
            framePosition = playing ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;
            beat = stepPosition % hitCount;
            for (int i = 0; i < hitCount; i++)
                getBeatStep(i)[activeKey] ^= received(steps[i]);
            // set
            if (received(track))
                activeTrack = current.track - 1;
            if (received(trackType))
                tracks[activeTrack].type = static_cast<Type>(current.trackType - 1);
            if (received(length))
                tracks[activeTrack].length = current.length - 1;
            if (received(sounds)) {
                tracks[activeTrack].sounds = current.sounds - 1;
                updateActiveSample();
            }
            if (received(root))
                root = current.root - 1;
            if (received(scale))
                scale = current.scale - 1;
            if (received(octave))
                tracks[activeTrack].octave = current.octave - 1;
            if (received(key)) {
                activeKey = current.key - 1;
                updateActiveSample();
                liveKey();
            }
            // custom
            if (received(bpm))
                bpm = current.bpm;
#undef received

            activePage = getBeatPage();
            activeLength = tracks[activeTrack].length;
            activeTrackType = tracks[activeTrack].type;
            activeSounds = tracks[activeTrack].sounds;
            activeOctave = tracks[activeTrack].octave;
            for (int s = 0; s < hitCount; s++)
                activeSteps[s] = getBeatStep(s)[activeKey];

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
