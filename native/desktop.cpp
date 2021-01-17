#include <array>
#include <cassert>
#include <filesystem>
#include <iostream>
#include <vector>

#include "webview/webview.h"

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"

#define SAMPLE_RATE 44100
#include "faust/dsp/one-sample-dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/UI.h"

#include "effects.h"
#include "sequencer.h"

struct UserData {
    groovebox::Input* input;
    groovebox::Sequencer* sequencer;
    groovebox::Effects* effects;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    int intControls[userData->effects->getNumIntControls()];
    float floatControls[userData->effects->getNumRealControls()];
    userData->effects->control(intControls, floatControls);
    for (int frame = 0; frame < frameCount; frame++) {
        userData->sequencer->compute(*(userData->input), ((float*) input)[frame]);
        userData->effects->compute((float*) userData->sequencer->output.data(), ((float*) output) + frame*device->playback.channels, intControls, floatControls);
    }
}

void toUi(webview::webview* view, std::string label, int* state) {
    view->bind("toUi:" + label, [state](std::string data) -> std::string {
        return std::to_string(*state);
    });
}

void fromUi(webview::webview* view, std::string label, int* input) {
    view->bind("fromUi:" + label, [input](std::string data) -> std::string {
        *input = std::stoi(data.substr(1, data.length() - 2));
        return "";
    });
}

void syncUi(webview::webview* view, std::string label, int* state, int* input) {
    fromUi(view, label, input);
    toUi(view, label, state);
}

int main(int argc, char* argv[]) { // TODO WinMain, see webview README
    ma_context context;
    ma_device_id* captureDeviceId = nullptr;
    ma_device_id* playbackDeviceId = nullptr;
    assert(ma_context_init(NULL, 0, NULL, &context) == MA_SUCCESS);
    if (argc >= 2) {
        ma_uint32 captureDeviceCount;
        ma_device_info* captureDeviceInfo;
        ma_uint32 playbackDeviceCount;
        ma_device_info* playbackDeviceInfo;
        assert(ma_context_get_devices(&context, &playbackDeviceInfo, &playbackDeviceCount,  &captureDeviceInfo, &captureDeviceCount) == MA_SUCCESS);
        for (int i = 0; i < captureDeviceCount; ++i)
            if (strcmp(argv[1], captureDeviceInfo[i].name) == 0)
                captureDeviceId = &captureDeviceInfo[i].id;
        for (int i = 0; i < playbackDeviceCount; ++i)
            if (strcmp(argv[argc > 2 ? 2 : 1], playbackDeviceInfo[i].name) == 0)
                playbackDeviceId = &playbackDeviceInfo[i].id;
        assert(captureDeviceId != nullptr);
        assert(playbackDeviceId != nullptr);
    }

    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    groovebox::Input input {};
    groovebox::Sequencer sequencer {};
    groovebox::Effects effects {};

    assert(sizeof(sequencer.output) == effects.getNumInputs() * sizeof(float));
    sequencer.init(root);
    effects.init(SAMPLE_RATE);
    UserData userData { &input, &sequencer, &effects };

    ma_device device;
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = 1;
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.capture.pDeviceID = captureDeviceId;
    deviceConfig.playback.channels = effects.getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.playback.pDeviceID = playbackDeviceId;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;
    deviceConfig.pUserData = &userData;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see docs/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (root / "ui" / "index.html").string());

    toUi(&view, "beat", &sequencer.beat);
    toUi(&view, "page", &sequencer.page);
    syncUi(&view, "tempo", &sequencer.active.tempo, &input.tempo);
    syncUi(&view, "play", &sequencer.active.play, &input.play);
    syncUi(&view, "record", &sequencer.active.record, &input.record);
    syncUi(&view, "clock", &sequencer.active.clock, &input.clock);
    syncUi(&view, "track", &sequencer.active.track, &input.track);
    syncUi(&view, "source", &sequencer.active.source, &input.source);
    syncUi(&view, "polyphonic", &sequencer.active.polyphonic, &input.polyphonic);
    syncUi(&view, "length", &sequencer.active.length, &input.length);
    syncUi(&view, "sounds", &sequencer.active.sounds, &input.sounds);
    syncUi(&view, "root", &sequencer.active.root, &input.root);
    syncUi(&view, "scale", &sequencer.active.scale, &input.scale);
    syncUi(&view, "octave", &sequencer.active.octave, &input.octave);
    syncUi(&view, "volume", &sequencer.active.volume, &input.volume);
    syncUi(&view, "pan", &sequencer.active.pan, &input.pan);
    syncUi(&view, "filter", &sequencer.active.filter, &input.filter);
    syncUi(&view, "resonance", &sequencer.active.resonance, &input.resonance);
    syncUi(&view, "delay", &sequencer.active.delay, &input.delay);
    syncUi(&view, "reverb", &sequencer.active.reverb, &input.reverb);
    for (int i = 0; i < sequencer.active.keys.size(); i++)
        syncUi(&view, "key:" + std::to_string(i), sequencer.active.keys.data() + i, input.keys.data() + i);
    for (int i = 0; i < sequencer.active.steps.size(); i++)
        syncUi(&view, "step:" + std::to_string(i), sequencer.active.steps.data() + i, input.steps.data() + i);
    for (int i = 0; i < sequencer.active.mutes.size(); i++)
        syncUi(&view, "mute:" + std::to_string(i), sequencer.active.mutes.data() + i, input.mutes.data() + i);

#ifdef WEBVIEW_COCOA
    auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 251/255.0, 241/255.0, 199/255.0, 1.0); // see docs/frameless.md
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
    ma_context_uninit(&context);
    return 0;
}
