#include <array>
#include <cmath>

namespace groovebox {
    const int trackCount = 8;
    const int hitCount = 16;
    const int stepCount = 128;
    const int keyCount = 15;
    const int sampleCount = 256;
    const int outputCount = 3;

    const std::array<std::string, 13> enferKits {
        "tr808", "tr909", "dmx", "dnb", "dark", "deep", "tech",
        "modular", "gabber", "bergh", "vermona", "commodore", "dmg",
    };

    const std::array<std::string, 18> enferSamples {
        "kick", "kick-up", "kick-down", "tom", "snare", "snare-up", "snare-down", "clap",
        "hat", "hat-open", "hat-shut", "cymb", "fx1", "fx2", "fx3", "fx4", "synth-C2", "synth-C3"
    };

    const std::array<std::array<int, keyCount>, 12> scaleOffsets {
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

    enum Type {
        kit,
        mono,
        poly,
        arp,
        chord,
        inputThrough,
    };

    struct Controls {
        int velocity = 10;
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
        int memoryIndex;
        Controls input;
        std::array<float, 5*SAMPLE_RATE> memory;
        std::array<Controls, sampleCount> samples;
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
        left,
        right,
        controls,
    };

    struct Sequencer {
        Input active { .bpm = 120 };
        // enfer
        std::array<float*, enferKits.size() * enferSamples.size()> enferFrames;
        std::array<ma_uint64, enferKits.size() * enferSamples.size()> enferLengths;
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
        void init(std::filesystem::path root) {
            // load enfer sample frames
            for (int kit = 0; kit < enferKits.size(); kit++) {
                for (int sample = 0; sample < enferSamples.size(); sample++) {
                    auto i = kit * enferSamples.size() + sample;
                    auto filename = root / "native" / "Enfer" / "media" / enferKits[kit] / (enferSamples[sample] + ".wav");
                    unsigned int channels;
                    unsigned int sampleRate;
                    enferFrames[i] = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &enferLengths[i], NULL);
                    MA_ASSERT(enferFrames[i] != NULL);
                    MA_ASSERT(sampleRate == SAMPLE_RATE);
                    // convert mono to stereo
                    if (channels == 1) {
                        float* frames = new float[enferLengths[i] * 2];
                        for (int f = 0; f < enferLengths[i]; f++)
                            frames[2*f] = frames[2*f + 1] = enferFrames[i][f];
                        MA_FREE(enferFrames[i]);
                        enferFrames[i] = frames;
                    }
                }
            }
        }

        void compute(Input current, float audio) {
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
                    liveKey(audio);
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
                getActiveControls()->velocity = current.velocity - 1;
            if (received(pan))
                getActiveControls()->pan = current.pan - 1;
            if (received(filter))
                getActiveControls()->filter = current.filter - 1;
            if (received(resonance))
                getActiveControls()->resonance = current.resonance - 1;
            if (received(reverb))
                getActiveControls()->reverb = current.reverb - 1;
            if (received(delay))
                getActiveControls()->delay = current.delay - 1;
            // custom
            if (received(bpm))
                active.bpm = current.bpm;
#undef received

            page = getBeatPage();
            active.length = tracks[active.track].length;
            active.type = tracks[active.track].type;
            active.sounds = tracks[active.track].sounds;
            active.octave = tracks[active.track].octave;
            active.velocity = getActiveControls()->velocity;
            active.pan = getActiveControls()->pan;
            active.filter = getActiveControls()->filter;
            active.resonance = getActiveControls()->resonance;
            active.reverb = getActiveControls()->reverb;
            active.delay = getActiveControls()->delay;
            for (int k = 0; k < keyCount; k++)
                active.keys[k] = k == currentKey;
            for (int s = 0; s < hitCount; s++)
                active.steps[s] = getBeatStep(s)[currentKey];
            for (int t = 0; t < trackCount; t++)
                active.mutes[t] = tracks[t].muted;

            // for (int t = 0; t < trackCount; t++)
            //     for (int k = 0; k < keyCount; k++)
            //         voiceOut[t][k][Output::position] += voiceIncrements[t][k];

            for (int t = 0; t < trackCount; t++)
                if (tracks[t].type >= Type::inputThrough)
                    useVoice(t, 0, audio);

            if (active.play && stepPosition != inSteps(framePosition - 1) % stepCount)
                for (int t = 0; t < trackCount; t++)
                    for (int k = 0; k < keyCount; k++)
                        if (getAbsoluteStep(t, stepPosition)[k])
                            useVoice(t, k, audio);

            previous = current;
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60.f * SAMPLE_RATE / active.bpm) * subdivision * 2);
        }

        void updateActiveSample() {
            if (tracks[active.track].type == Type::kit)
                tracks[active.track].currentSample = getSample(active.track, currentKey);
        }

        int getSample(int t, int key) {
            return tracks[t].sounds > 12 ? key*18 + tracks[t].sounds + 2 : key + tracks[t].sounds*18;
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

        Controls* getActiveControls() {
            return tracks[active.track].type >= Type::inputThrough
                ? &tracks[active.track].input
                : &tracks[active.track].samples[tracks[active.track].currentSample];
        }

        void liveKey(float audio) {
            if (active.play && active.arm) {
                auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                getAbsoluteStep(active.track, quantizePosition)[currentKey] = true;
                if (stepPosition != quantizePosition)
                    return; // prevent double-trig -- aka live-quantize
            }
            useVoice(active.track, currentKey, audio);
        }

        void useVoice(int t, int key, float audio) {
            switch (tracks[t].type) {
            case Type::kit:
                // auto s = getSample(t, key);
                // voiceOut[t][key][Output::left] = audio;
                // voiceOut[t][key][Output::right] = audio;
                // voiceOut[t][key][Output::controls] = encodeControls(tracks[t].samples[s]);
                // voiceIncrements[t][key] = 1;
                break;
            case Type::mono:
                // auto s = tracks[t].currentSample;
                // voiceOut[t][0][Output::left] = audio;
                // voiceOut[t][0][Output::right] = audio;
                // voiceOut[t][0][Output::controls] = encodeControls(tracks[t].samples[s]);
                // voiceIncrements[t][key] = noteIncrement(t, key);
                break;
            case Type::poly:
                // auto s = tracks[t].currentSample;
                // voiceOut[t][key][Output::left] = audio;
                // voiceOut[t][key][Output::right] = audio;
                // voiceOut[t][key][Output::controls] = encodeControls(tracks[t].samples[s]);
                // voiceIncrements[t][key] = noteIncrement(t, key);
                break;
            case Type::arp:
                // TODO
                break;
            case Type::chord:
                // TODO
                break;
            case Type::inputThrough:
                voiceOut[t][0][Output::left] = audio;
                voiceOut[t][0][Output::right] = audio;
                voiceOut[t][0][Output::controls] = encodeControls(tracks[t].input);
                break;
            }
        }

        int encodeControls(Controls controls) {
            return controls.velocity
                | (controls.pan       << 4)
                | (controls.filter    << 8)
                | (controls.resonance << 12)
                | (controls.reverb    << 16)
                | (controls.delay     << 20);
        }

        float noteIncrement(int t, int key) {
            auto note = active.root + scaleOffsets[active.scale][key] + tracks[t].octave*12;
            return pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
        }
    };
}
