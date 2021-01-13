#include <array>
#include <cmath>

namespace groovebox {
    const int trackCount = 8;
    const int hitCount = 16;
    const int stepCount = 128;
    const int keyCount = 15;

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

    struct Output {
        float l;
        float r;
        float controls;
    };

    struct Sample {
        bool stereo;
        ma_uint64 length;
        float* frames;
    };

    struct Controls {
        int velocity = 10;
        int pan = 7;
        int filter = 7;
        int resonance;
        int reverb;
        int delay;

        void encode(Output& output) {
            output.controls = velocity | pan << 4 | filter << 8 | resonance << 12 | reverb << 16 | delay << 20;
        }
    };

    struct Voice {
        float position;
        float increment;

        void play(Sample sample, Output& output) {
            output.l = interpolate(sample, sample.stereo ? 2*position : position);
            output.r = interpolate(sample, sample.stereo ? 2*position + 1 : position);
            position += increment;
        }

        float interpolate(Sample sample, float frame) {
            auto i = int(frame);
            return position+1 >= sample.length
                ? 0
                : sample.frames[i] + (frame-i) * (sample.frames[i+1] - sample.frames[i]);
        }
    };

    enum Type {
        kit,
        mono,
        poly,
        arp,
        chord,
        line,
    };

    struct Track {
        Type type;
        int length;
        int sounds;
        int octave = 3;
        int muted;
        int currentKitKey;
        Controls inputControls;
        std::array<Controls, enferKits.size() * enferSamples.size()> sampleControls;
        std::array<std::array<bool, keyCount>, stepCount> steps;
        std::array<Voice, keyCount> voices;
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

    struct Sequencer {
        Input active { .bpm = 120 };
        int framePosition;
        int stepPosition;
        int beat;
        int page;
        int currentKey;
        Input previous;
        std::array<Track, trackCount> tracks;
        std::array<Sample, enferKits.size() * enferSamples.size()> library;
        std::array<std::array<Output, keyCount>, trackCount> output;

        // explicit `init` so that we keep the default, zero-initializing constructor
        void init(std::filesystem::path root) {
            for (int kit = 0; kit < enferKits.size(); kit++) {
                for (int sample = 0; sample < enferSamples.size(); sample++) {
                    auto i = kit * enferSamples.size() + sample;
                    auto filename = root / "native" / "Enfer" / "media" / enferKits[kit] / (enferSamples[sample] + ".wav");
                    unsigned int channels;
                    unsigned int sampleRate;
                    library[i].frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &library[i].length, NULL);
                    assert(library[i].frames != NULL);
                    assert(sampleRate == SAMPLE_RATE);
                    assert(channels == 1 || channels == 2);
                    library[i].stereo = channels == 2;
                }
            }
        }

        void compute(Input current, float audio) {
            // input processing macros
#define received(x) current.x && current.x != previous.x
#define set(recipient, x, type) if (received(x)) (recipient).x = static_cast<type>(current.x - 1)

            // input toggles
            active.play ^= received(play);
            active.arm ^= received(arm);
            framePosition = active.play ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;
            beat = stepPosition % hitCount;
            for (int k = 0; k < keyCount; k++)
                if(received(keys[k])) {
                    currentKey = k;
                    if (tracks[active.track].type == Type::kit)
                        tracks[active.track].currentKitKey = k;
                    if (active.play && active.arm) {
                        auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                        getAbsoluteStep(active.track, quantizePosition)[currentKey] = true;
                        if (stepPosition != quantizePosition)
                            current.keys[k] = 0; // prevent double-trig -- aka live-quantize
                    }
                }
            for (int h = 0; h < hitCount; h++)
                getBeatStep(h)[currentKey] ^= received(steps[h]);
            for (int t = 0; t < trackCount; t++)
                tracks[t].muted ^= received(mutes[t]);

            // input values
            set(active, bpm, int);
            set(active, root, int);
            set(active, scale, int);
            set(active, track, int);
            set(tracks[active.track], type, Type);
            set(tracks[active.track], length, int);
            set(tracks[active.track], sounds, int);
            set(tracks[active.track], octave, int);
            set(*getActiveControls(), velocity, int);
            set(*getActiveControls(), pan, int);
            set(*getActiveControls(), filter, int);
            set(*getActiveControls(), resonance, int);
            set(*getActiveControls(), reverb, int);
            set(*getActiveControls(), delay, int);

            // sync active, which reflects state to ui
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

            // play track voices
            auto playing = active.play && stepPosition != inSteps(framePosition - 1) % stepCount;
            for (int t = 0; t < tracks.size(); t++)
                for (int v = 0; v < tracks[t].voices.size(); v++)
                    useVoice(t, v, audio, playing && getAbsoluteStep(t, stepPosition)[v] || t == active.track && received(keys[v]));

            // remember previous for next call
            previous = current;
#undef set
#undef received
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60.f * SAMPLE_RATE / active.bpm) * subdivision * 2);
        }

        int getSample(int t, int key) {
            return std::min(
                int(library.size() - 1),
                tracks[t].sounds > 12 ? key*18 + tracks[t].sounds + 2 : key + tracks[t].sounds*18
            );
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
            return tracks[active.track].type >= Type::line
                ? &tracks[active.track].inputControls
                : &tracks[active.track].sampleControls[getSample(active.track, tracks[active.track].currentKitKey)];
        }

        void useVoice(int t, int key, float audio, bool fresh) {
            int s;
            switch (tracks[t].type) {
            case Type::kit:
                if (fresh) {
                    tracks[t].voices[key].position = 0;
                    tracks[t].voices[key].increment = 1;
                }
                s = getSample(t, key);
                tracks[t].voices[key].play(library[s], output[t][key]);
                tracks[t].sampleControls[s].encode(output[t][key]);
                break;
            case Type::mono:
                if (fresh) {
                    tracks[t].voices[0].position = 0;
                    tracks[t].voices[0].increment = noteIncrement(t, key);
                }
                s = getSample(t, tracks[t].currentKitKey);
                tracks[t].voices[0].play(library[s], output[t][0]);
                tracks[t].sampleControls[s].encode(output[t][0]);
                break;
            case Type::poly:
                if (fresh) {
                    tracks[t].voices[key].position = 0;
                    tracks[t].voices[key].increment = noteIncrement(t, key);
                }
                s = getSample(t, tracks[t].currentKitKey);
                tracks[t].voices[key].play(library[s], output[t][key]);
                tracks[t].sampleControls[s].encode(output[t][key]);
                break;
            case Type::arp:
                // TODO
                break;
            case Type::chord:
                // TODO
                break;
            case Type::line:
                output[t][0].l = audio;
                output[t][0].r = audio;
                tracks[t].inputControls.encode(output[t][0]);
                break;
            }
        }

        float noteIncrement(int t, int key) {
            auto note = active.root + scaleOffsets[active.scale][key] + tracks[t].octave*12;
            return pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
        }
    };
}
