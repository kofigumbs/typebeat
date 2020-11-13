#include <filesystem>

#include "enfer.hpp"

#include "webview/webview.h"

#include "faust/dsp/dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/UI.h"
#include "faust/gui/Soundfile.h"
#include "faust/dsp/cpp-dsp-adapter.h"

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
        unsigned int channels[fileCount];
        Soundfile* soundfile = new Soundfile();
        soundfile->fChannels = 2;

        for (int kit = 0; kit < enfer::kits.size(); kit++) {
            for (int sample = 0; sample < enfer::samples.size(); sample++) {
                auto i = kit * sample;
                auto filename = root / "dsp" / "Enfer" / "media" / enfer::kits[kit] / (enfer::samples[sample] + ".wav");
                unsigned int sampleRate;
                ma_uint64 length;
                data[i] = drwav_open_file_and_read_pcm_frames_f32(filename.c_str(), &channels[i], &sampleRate, &length, NULL);
                MA_ASSERT(data[i] != NULL);
                soundfile->fSR[i] = sampleRate;
                soundfile->fOffset[i] = totalLength;
                soundfile->fLength[i] = length;
                totalLength += length;
            }
        }

        for (int i = fileCount; i < MAX_SOUNDFILE_PARTS; i++) {
            soundfile->fLength[i] = BUFFER_SIZE;
            soundfile->fSR[i] = SAMPLE_RATE;
            soundfile->fOffset[i] = totalLength;
            totalLength += BUFFER_SIZE;
        }

        soundfile->fBuffers = new float*[soundfile->fChannels];
        for (int channel = 0; channel < soundfile->fChannels; channel++)
            soundfile->fBuffers[channel] = new float[totalLength] {};
        for (int i = 0; i < fileCount; i++) {
            for (int channel = 0; channel < channels[i]; channel++)
                for (int frame = 0; frame < *(soundfile->fLength); frame++)
                    soundfile->fBuffers[channel][frame] = data[i][channel + frame * channels[i]];
            free(data[i]);
        }
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
    auto mydsp = (dsp*) device->pUserData;
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
    mydsp->compute(frameCount, inputChannels, outputChannels);

    // interleave output audio frames
    for (int channel = 0; channel < device->playback.channels; channel++)
        for (int frame = 0; frame < frameCount; frame++)
            ((float*) output)[channel + frame*device->playback.channels] = deinterleavedOutput[channel][frame];
}

#ifdef WIN32
int WINAPI WinMain(HINSTANCE hInt, HINSTANCE hPrevInst, LPSTR lpCmdLine, int nCmdShow) {
#else
int main(int argc, char* argv[]) {
#endif
    dsp* mydsp = createmydsp();
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = mydsp->getNumInputs();
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.playback.channels = mydsp->getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.periodSizeInFrames = 256;
    deviceConfig.dataCallback = callback;

    ma_device device;
    MA_ASSERT(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);
    mydsp->init(device.sampleRate);
    device.pUserData = mydsp;

    auto root = std::filesystem::absolute(std::filesystem::path(argv[0]))
        .parent_path() // build directory
        .parent_path(); // project directory
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (root / "ui" / "index.html").string());
    mydsp->buildUserInterface(new WebviewUI(&view, root));

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
