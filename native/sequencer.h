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

    struct Sample {
        int velocity;
        int pan = 7;
        int filter = 7;
        int resonance;
        int reverb;
        int delay;
    };

    struct Track {
        Type type;
        int length;
        int currentSample;
        int sounds;
        int octave = 3;
        int muted;
        std::array<Sample, keyCount> samples;
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    struct Input {
        int play;
        int arm;
        int bpm;
        int root;
        int scale;
        int track;
        int type;
        int length;
        int sounds;
        int octave;
        int velocity;
        int pan;
        int filter;
        int resonance;
        int reverb;
        int delay;
        std::array<int, keyCount> keys;
        std::array<int, hitCount> steps;
        std::array<int, trackCount> mutes;
    };

    enum Output {
        sample,
        position
    };

    struct Sequencer {
        Input active;
        // transport
        int framePosition;
        int stepPosition;
        int beat;
        int page;
        // internals
        int currentKey;
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
            active.bpm = 120;
            updateActive();
        }

        void compute(Input current) {
#define received(x) current.x && current.x != previous.x
            // toggle
            active.play ^= received(play);
            active.arm ^= received(arm);
            framePosition = active.play ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;
            beat = stepPosition % hitCount;
            for (int k = 0; k < keyCount; k++)
                if(received(keys[k])) {
                    currentKey = k;
                    updateActiveSample();
                    liveKey();
                }
            for (int h = 0; h < hitCount; h++)
                getBeatStep(h)[currentKey] ^= received(steps[h]);
            for (int t = 0; t < trackCount; t++)
                tracks[t].muted ^= received(mutes[t]);
            // set
            if (received(track))
                active.track = current.track - 1;
            if (received(type))
                tracks[active.track].type = static_cast<Type>(current.type - 1);
            if (received(length))
                tracks[active.track].length = current.length - 1;
            if (received(sounds)) {
                tracks[active.track].sounds = current.sounds - 1;
                updateActiveSample();
            }
            if (received(root))
                active.root = current.root - 1;
            if (received(scale))
                active.scale = current.scale - 1;
            if (received(octave))
                tracks[active.track].octave = current.octave - 1;
            if (received(velocity))
                tracks[active.track].samples[tracks[active.track].currentSample].velocity = current.velocity - 1;
            if (received(pan))
                tracks[active.track].samples[tracks[active.track].currentSample].pan = current.pan - 1;
            if (received(filter))
                tracks[active.track].samples[tracks[active.track].currentSample].filter = current.filter - 1;
            if (received(resonance))
                tracks[active.track].samples[tracks[active.track].currentSample].resonance = current.resonance - 1;
            if (received(reverb))
                tracks[active.track].samples[tracks[active.track].currentSample].reverb = current.reverb - 1;
            if (received(delay))
                tracks[active.track].samples[tracks[active.track].currentSample].delay = current.delay - 1;
            // custom
            if (received(bpm))
                active.bpm = current.bpm;
#undef received

            page = getBeatPage();
            updateActive();

            for (int t = 0; t < trackCount; t++)
                for (int k = 0; k < keyCount; k++)
                    voiceOut[t][k][Output::position] += voiceIncrements[t][k];
            if (active.play && stepPosition != inSteps(framePosition - 1) % stepCount)
                for (int t = 0; t < trackCount; t++)
                    for (int k = 0; k < keyCount; k++)
                        if (getAbsoluteStep(t, stepPosition)[k])
                            useVoice(t, k);

            previous = current;
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60.f * SAMPLE_RATE / active.bpm) * subdivision * 2);
        }

        void updateActive() {
            active.length = tracks[active.track].length;
            active.type = tracks[active.track].type;
            active.sounds = tracks[active.track].sounds;
            active.octave = tracks[active.track].octave;
            active.velocity = tracks[active.track].samples[tracks[active.track].currentSample].velocity;
            active.pan = tracks[active.track].samples[tracks[active.track].currentSample].pan;
            active.filter = tracks[active.track].samples[tracks[active.track].currentSample].filter;
            active.resonance = tracks[active.track].samples[tracks[active.track].currentSample].resonance;
            active.reverb = tracks[active.track].samples[tracks[active.track].currentSample].reverb;
            active.delay = tracks[active.track].samples[tracks[active.track].currentSample].delay;
            for (int k = 0; k < keyCount; k++)
                active.keys[k] = k == currentKey;
            for (int s = 0; s < hitCount; s++)
                active.steps[s] = getBeatStep(s)[currentKey];
            for (int t = 0; t < trackCount; t++)
                active.mutes[t] = tracks[t].muted;
        }

        void updateActiveSample() {
            if (tracks[active.track].type == Type::kit)
                tracks[active.track].currentSample = getSample(active.track, currentKey);
        }

        int getSample(int t, int key) {
            auto track = tracks[t];
            return track.sounds > 12 ? key*18 + track.sounds + 2 : key + track.sounds*18;
        }

        int getBeatPage() {
            return (stepPosition / hitCount) % (tracks[active.track].length + 1);
        }

        bool* getBeatStep(int i) {
            return tracks[active.track].steps[i + getBeatPage() * hitCount].data();
        }

        bool* getAbsoluteStep(int t, int i) {
            return tracks[t].steps[i % ((tracks[t].length + 1) * hitCount)].data();
        }

        void liveKey() {
            if (active.play && active.arm) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                getAbsoluteStep(active.track, quantizePosition)[currentKey] = true;
                if (stepPosition != quantizePosition)
                    return; // prevent double-trig -- aka live-quantize
            }
            useVoice(active.track, currentKey);
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
            voiceOut[t][voiceIndex][Output::sample] = track.type == Type::kit ? getSample(t, key) : track.currentSample;
            switch (tracks[t].type) {
            case Type::kit:
                voiceIncrements[t][voiceIndex] = 1;
                break;
            default:
                auto note = active.root + scaleOffsets[active.scale][key] + track.octave*12;
                voiceIncrements[t][voiceIndex] = pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
                break;
            }
        }
    };
}
