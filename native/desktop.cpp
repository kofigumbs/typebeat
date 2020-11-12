#include <filesystem>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_DECODING
#define MA_NO_GENERATION

#include "faust/dsp/dsp.h"
#include "faust/gui/UI.h"
#include "faust/dsp/cpp-dsp-adapter.h"
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"

struct WebviewUI: UI {
    webview::webview* view;
    WebviewUI(webview::webview* view): view(view) {}

    void openTabBox(const char* label) override {}
    void openHorizontalBox(const char* label) override {}
    void openVerticalBox(const char* label) override {}
    void closeBox() override {}
    void declare(float* zone, const char* key, const char* val) override {}

    void addSoundfile(const char* label, const char* filename, Soundfile** sf_zone) override {
        // TODO
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

    auto path = std::filesystem::absolute(std::filesystem::path(argv[0]))
        .parent_path() // build directory
        .parent_path(); // project directory
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (path / "ui" / "index.html").string());
    mydsp->buildUserInterface(new WebviewUI(&view));

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
