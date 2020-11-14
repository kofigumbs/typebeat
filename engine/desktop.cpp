#include <filesystem>

#include "enfer.hpp"

#include "webview/webview.h"

#include "faust/dsp/dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/UI.h"
#include "faust/gui/Soundfile.h"

Soundfile* defaultsound;
#include "mydsp.h"

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"


struct WebviewUI: UI {
    webview::webview* view;
    std::filesystem::path root;
    WebviewUI(webview::webview* view, std::filesystem::path root): view(view), root(root) {}

    void openTabBox(const char* label) override {}
    void openHorizontalBox(const char* label) override {}
    void openVerticalBox(const char* label) override {}
    void closeBox() override {}
    void declare(float* zone, const char* key, const char* val) override {}

    void addSoundfile(const char* label, const char* filename, Soundfile** sf_zone) override {
        const int fileCount = enfer::samples.size() * enfer::kits.size();
        MA_ASSERT(fileCount <= MAX_SOUNDFILE_PARTS);

        int totalLength = 0;
        float* data[fileCount];
        unsigned int fileChannels[fileCount];
        Soundfile* soundfile = new Soundfile();
        soundfile->fChannels = 2;

        // read each enfer wav file into `data`, tracking metadata in `soundfile` and `fileChannels`
        for (int kit = 0; kit < enfer::kits.size(); kit++) {
            for (int sample = 0; sample < enfer::samples.size(); sample++) {
                auto i = kit * enfer::samples.size() + sample;
                auto filename = root / "engine" / "Enfer" / "media" / enfer::kits[kit] / (enfer::samples[sample] + ".wav");
                unsigned int sampleRate;
                ma_uint64 length;
                data[i] = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &fileChannels[i], &sampleRate, &length, NULL);
                MA_ASSERT(data[i] != NULL);
                soundfile->fSR[i] = sampleRate;
                soundfile->fOffset[i] = totalLength;
                soundfile->fLength[i] = length;
                totalLength += length;
            }
        }

        // fill metadata for remaining soundfile parts
        for (int i = fileCount; i < MAX_SOUNDFILE_PARTS; i++) {
            soundfile->fLength[i] = BUFFER_SIZE;
            soundfile->fSR[i] = SAMPLE_RATE;
            soundfile->fOffset[i] = totalLength;
            totalLength += BUFFER_SIZE;
        }

        // fill actual audio data, now that we know the total buffer size
        soundfile->fBuffers = new float*[soundfile->fChannels];
        for (int channel = 0; channel < soundfile->fChannels; channel++)
            soundfile->fBuffers[channel] = new float[totalLength] {};
        for (int i = 0; i < fileCount; i++) {
            for (int channel = 0; channel < soundfile->fChannels; channel++)
                if (fileChannels[i] == 1)
                    memcpy(soundfile->fBuffers[channel] + soundfile->fOffset[i], data[i], sizeof(float) * soundfile->fLength[i]);
                else
                    for (int frame = 0; frame < soundfile->fLength[i]; frame++)
                        soundfile->fBuffers[channel][soundfile->fOffset[i] + frame] = data[i][channel + frame * fileChannels[i]];
            MA_FREE(data[i]);
        }

        *(sf_zone) = soundfile;
    }

    void toUi(const char* label, float* zone) {
        view->bind(label, [this, zone](std::string data) -> std::string {
            return std::to_string(*zone);
        });
    }
    void addHorizontalBargraph(const char* label, float* zone, float min, float max) override { toUi(label, zone); }
    void addVerticalBargraph(const char* label, float* zone, float min, float max) override { toUi(label, zone); }

    void toDsp(const char* label, float* zone) {
        view->bind(label, [this, zone](std::string data) -> std::string {
            *zone = std::stof(data.substr(1, data.length() - 2));
            return "";
        });
    }
    void addCheckButton(const char* label, float* zone) override { toDsp(label, zone); }
    void addVerticalSlider(const char* label, float* zone, float init, float min, float max, float step) override { toDsp(label, zone); }
    void addHorizontalSlider(const char* label, float* zone, float init, float min, float max, float step) override { toDsp(label, zone); }
    void addNumEntry(const char* label, float* zone, float init, float min, float max, float step) override { toDsp(label, zone); }
    void addButton(const char* label, float* zone) override { toDsp(label, zone); }
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto DSP = (dsp*) device->pUserData;
    float deinterleavedInput[device->capture.channels][frameCount],
    deinterleavedOutput[device->playback.channels][frameCount],
    *inputChannels[device->capture.channels],
    *outputChannels[device->playback.channels];

    // de-interleave input audio frames and setup input wrapping array
    for (int channel = 0; channel < device->capture.channels; channel++) {
        for (int frame = 0; frame < frameCount; frame++)
            deinterleavedInput[channel][frame] = ((float*) input)[channel + frame*device->capture.channels];
        inputChannels[channel] = ((float*) deinterleavedInput) + channel*frameCount;
    }

    // setup output wrapping array
    for (int channel = 0; channel < device->playback.channels; channel++)
        outputChannels[channel] = ((float*) deinterleavedOutput) + channel*frameCount;

    // render audio context
    DSP->compute(frameCount, inputChannels, outputChannels);

    // interleave output audio frames
    for (int channel = 0; channel < device->playback.channels; channel++)
        for (int frame = 0; frame < frameCount; frame++)
            ((float*) output)[channel + frame*device->playback.channels] = deinterleavedOutput[channel][frame];
}

int main(int argc, char* argv[]) { // TODO WinMain, see webview README
    dsp* DSP = new mydsp();
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = DSP->getNumInputs();
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.playback.channels = DSP->getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;

    ma_device device;
    MA_ASSERT(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);
    DSP->init(device.sampleRate);
    device.pUserData = DSP;

    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (root / "ui" / "index.html").string());
    DSP->buildUserInterface(new WebviewUI(&view, root));

#ifdef WEBVIEW_COCOA
    auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 251/255.0, 241/255.0, 199/255.0, 1.0); // see notes/frameless.md
    auto window = (id) view.window();
    objc_msgSend(window, sel_registerName("setBackgroundColor:"), light);
    objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
    objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
    objc_msgSend(window, sel_registerName("center"));
    objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif

    ma_device_start(&device);
    view.run();
    ma_device_uninit(&device);
    return 0;
}
