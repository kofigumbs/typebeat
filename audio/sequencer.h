#include "choc/containers/choc_SingleReaderSingleWriterFIFO.h"

namespace groovebox {
    const int trackCount = 15;
    const int hitCount = 16;
    const int stepCount = 128;
    const int keyCount = 15;

    const std::array<std::string, 13> enferKits {
        "tr808", "tr909", "dmx", "dnb", "dark", "deep", "tech",
        "modular", "gabber", "bergh", "vermona", "commodore", "dmg",
    };

    const std::array<std::string, 17> enferSamples {
        "kick", "kick-up", "kick-down", "tom", "snare", "snare-up", "snare-down", "clap",
        "hat", "hat-open", "hat-shut", "cymb", "fx1", "fx2", "fx3", "fx4", "synth-C3"
    };

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

    struct Sample {
        bool stereo;
        ma_uint64 length;
        float* frames;

        float left(int i) {
            return frames[stereo ? 2*i : i];
        }

        float right(int i) {
            return frames[stereo ? 2*i + 1 : i];
        }
    };

    struct Library {
        std::array<Sample, enferKits.size() * enferSamples.size()> samples;
    };

    struct Output {
        // eventually cast to a float*, so fields should only be floats
        float l;
        float r;
        float controls;
    };

    struct Controls {
        int volume = 7;
        int pan = 7;
        int filter = 7;
        int resonance;
        int delay;
        int reverb;

        void encode(Output& output) {
            output.controls = volume | pan << 4 | filter << 8 | resonance << 12 | delay << 16 | reverb << 20;
        }
    };

    struct Voice {
        bool active;
        float position;
        float increment;

        void prepare(float increment_) {
            active = true;
            position = 0;
            increment = increment_;
        }

        void release() {
            active = false;
        }

        void play(Sample sample, Output& output) {
            auto i = int(position);
            if (active && position == i && position < sample.length) {
                output.l = sample.left(i);
                output.r = sample.right(i);
                position += increment;
            }
            else if (active && i + 1 < sample.length) {
                output.l = interpolate(position-i, sample.left(i), sample.left(i + 1));
                output.r = interpolate(position-i, sample.right(i), sample.right(i + 1));
                position += increment;
            }
            else {
                active = false;
                output.l = output.r = 0;
            }
        }

        float interpolate(float x, float a, float b) {
            return a + x*(b - a);
        }
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
        Sample lineSample;
        Controls lineControls;
        std::array<Controls, enferKits.size() * enferSamples.size()> sampleControls;
        std::array<Voice, keyCount> voices;
        std::array<std::array<bool, keyCount>, stepCount> steps;
    };

    struct Input {
        int play;
        int record;
        int clear;
        int tempo;
        int root;
        int scale;
        int track;
        int source;
        int polyphonic;
        int length;
        int samplePack;
        int octave;
        int volume;
        int pan;
        int filter;
        int resonance;
        int delay;
        int reverb;
        int selection;
        std::array<int, keyCount> keys;
        std::array<int, hitCount> steps;
        std::array<int, trackCount> mutes;
    };

    struct Sequencer {
        Input active { .tempo = 120 };
        int framePosition;
        int stepPosition;
        int beat;
        int page;
        Library library;
        std::array<Track, trackCount> tracks;
        std::array<std::array<Output, keyCount>, trackCount> output;
        std::array<bool, keyCount> receivedKeys;
        std::map<std::string, std::function<void(int)>> handlers;
        choc::fifo::SingleReaderSingleWriterFIFO<std::pair<std::function<void(int)>, int>> events;

        // explicit `init` so that we keep the default, zero-initializing constructor
        void init(std::filesystem::path root) {
            for (int t = 0; t < tracks.size(); t++) {
                tracks[t].lineSample.stereo = false;
                tracks[t].lineSample.length = 6*SAMPLE_RATE;
                tracks[t].lineSample.frames = new float[tracks[t].lineSample.length];
            }
            for (int kit = 0; kit < enferKits.size(); kit++) {
                for (int sample = 0; sample < enferSamples.size(); sample++) {
                    auto i = kit * enferSamples.size() + sample;
                    auto filename = root / "audio" / "Enfer" / "media" / enferKits[kit] / (enferSamples[sample] + ".wav");
                    unsigned int channels;
                    unsigned int sampleRate;
                    library.samples[i].frames = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels, &sampleRate, &library.samples[i].length, NULL);
                    assert(library.samples[i].frames != NULL);
                    assert(sampleRate == SAMPLE_RATE);
                    assert(channels == 1 || channels == 2);
                    library.samples[i].stereo = channels == 2;
                }
            }
            handlers["keyDown"] = [this](int value) {
                receivedKeys[value] = true;
                tracks[active.track].selection = value;
                if (tracks[active.track].source == Source::kit)
                    tracks[active.track].kitSelection = value;
                if (active.play && active.record) {
                    auto quantizePosition = (int) ((inSteps(framePosition, 2) + 1) / 2.0) % stepCount;
                    getAbsoluteStep(active.track, quantizePosition)[value] = true;
                    if (stepPosition != quantizePosition)
                        receivedKeys[value] = false; // prevent double-trig -- aka live-quantize
                }
            };
            handlers["track"] = [this](int value) {
                active.track = value;
            };
            handlers["samplePack"] = [this](int value) {
                tracks[active.track].samplePack = value;
            };
            handlers["mute"] = [this](int value) {
                tracks[value].muted ^= true;
            };
            events.reset(8);
        }

        void compute(float audio) {
            framePosition = active.play ? framePosition+1 : -1;
            stepPosition = inSteps(framePosition) % stepCount;
            beat = stepPosition % hitCount;
            std::pair<std::function<void(int)>, int> pair;
            while(events.pop(pair))
                pair.first(pair.second);
            auto playing = active.play && stepPosition != inSteps(framePosition - 1) % stepCount;
            for (int t = 0; t < tracks.size(); t++)
                for (int v = 0; v < tracks[t].voices.size(); v++)
                    setOutput(t, v, audio, t == active.track && receivedKeys[v] || playing && !tracks[t].muted && getAbsoluteStep(t, stepPosition)[v]);
            receivedKeys.fill(false);
        }

        int inSteps(int frames, int subdivision = 1) {
            return floor(frames / (60.f * SAMPLE_RATE / active.tempo) * subdivision * 2);
        }

        int getKitSample(int t, int key) {
            if (tracks[t].samplePack == 13) // fx4
                return std::min(int(library.samples.size() - 1), (key + 1)*17 - 2);
            else if (tracks[t].samplePack == 14) // synths
                return std::min(int(library.samples.size() - 1), (key + 1)*17 - 1);
            else
                return key + tracks[t].samplePack*17;
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
            switch (tracks[active.track].source) {
            case Source::kit:
                return &tracks[active.track].sampleControls[getKitSample(active.track, tracks[active.track].selection)];
            case Source::note:
                return &tracks[active.track].sampleControls[getKitSample(active.track, tracks[active.track].kitSelection)];
            case Source::lineThrough:
            case Source::lineRecord:
            case Source::linePlay:
                return &tracks[active.track].lineControls;
            }
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
            auto s = getKitSample(t, sampleKey);
            setOutputSampleAudio(t, key, library.samples[s], note, fresh);
            tracks[t].sampleControls[s].encode(output[t][key]);
        }

        void setOutputSampleAudio(int t, int key, Sample sample, int note, bool fresh) {
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
                active.root * (tracks[t].source != Source::kit) +
                scaleOffsets[active.scale][key % 7] +
                (tracks[t].octave + (key/7 - 1)) * 12;
            return pow(2.0f, note / 12.0f) / pow(2.0f, 36 / 12.0f);
        }
    };
}
