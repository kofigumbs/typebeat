#include <array>
#include <cassert>
#include <filesystem>
#include <vector>

#include "webview/webview.h"

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"

#include "faust/dsp/one-sample-dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/UI.h"
#include "faust/gui/Soundfile.h"

Soundfile* defaultsound;
#include "mydsp.h"

#include "enfer-ui.h"
#include "sequencer.h"

struct UserData {
    groovebox::Input* input;
    groovebox::Sequencer* sequencer;
    one_sample_dsp* dsp;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    int intControls[userData->dsp->getNumIntControls()];
    float floatControls[userData->dsp->getNumRealControls()];
    userData->dsp->control(intControls, floatControls);
    for (int frame = 0; frame < frameCount; frame++) {
        userData->sequencer->compute(*(userData->input));
        userData->dsp->compute((float*) userData->sequencer->voiceOut.data(), ((float*)output) + frame*device->playback.channels, intControls, floatControls);
    }
}

void toView(webview::webview* view, std::string label, int* p) {
    view->bind("groovebox:" + label, [p](std::string data) -> std::string {
        return std::to_string(*p);
    });
}

void toSequencer(webview::webview* view, std::string label, int* p) {
    view->bind("groovebox:" + label, [p](std::string data) -> std::string {
        *p = std::stoi(data.substr(1, data.length() - 2));
        return "";
    });
}

int main(int argc, char* argv[]) { // TODO WinMain, see webview README
    groovebox::Input input {};
    groovebox::Sequencer sequencer {};
    one_sample_dsp* dsp = new mydsp();
    assert(groovebox::trackCount * groovebox::keyCount * groovebox::Output::count == dsp->getNumInputs());
    UserData userData { &input, &sequencer, dsp };

    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_playback);
    deviceConfig.playback.channels = dsp->getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;

    ma_device device;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);
    dsp->init(device.sampleRate);
    device.pUserData = &userData;

    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (root / "ui" / "index.html").string());
    dsp->buildUserInterface(new groovebox::EnferUI(root));

    toView(&view, "playing", &(sequencer.playing));
    toView(&view, "armed", &(sequencer.armed));
    toView(&view, "track", &(sequencer.activeTrack));
    toView(&view, "lastKey", &(sequencer.lastKey));
    toView(&view, "beat", &(sequencer.stepPosition));
    toSequencer(&view, "play", &(input.playDown));
    toSequencer(&view, "arm", &(input.armDown));
    for (int i = 0; i < input.trackDown.size(); i++)
        toSequencer(&view, "track:" + std::to_string(i), input.trackDown.data() + i);
    for (int i = 0; i < input.keyDown.size(); i++)
        toSequencer(&view, "key:" + std::to_string(i), input.keyDown.data() + i);
    for (int i = 0; i < input.stepDown.size(); i++)
        toSequencer(&view, "step:" + std::to_string(i), input.stepDown.data() + i);

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
