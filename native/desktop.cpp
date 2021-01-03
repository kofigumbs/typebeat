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
#include "faust/gui/Soundfile.h" // defines SAMPLE_RATE

Soundfile* defaultsound; // required by sampler.h

#include "sampler.h"
#include "enfer-ui.h"
#include "sequencer.h"

struct UserData {
    groovebox::Input* input;
    groovebox::Sequencer* sequencer;
    groovebox::Sampler* sampler;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    int intControls[userData->sampler->getNumIntControls()];
    float floatControls[userData->sampler->getNumRealControls()];
    userData->sampler->control(intControls, floatControls);
    for (int frame = 0; frame < frameCount; frame++) {
        userData->sequencer->compute(*(userData->input));
        userData->sampler->compute((float*) userData->sequencer->voiceOut.data(), ((float*)output) + frame*device->playback.channels, intControls, floatControls);
    }
}

void toView(webview::webview* view, std::string label, int* p) {
    view->bind("toUi:" + label, [p](std::string data) -> std::string {
        return std::to_string(*p);
    });
}

void toSequencer(webview::webview* view, std::string label, int* p) {
    view->bind("fromUi:" + label, [p](std::string data) -> std::string {
        *p = std::stoi(data.substr(1, data.length() - 2));
        return "";
    });
}

template <std::size_t N>
void toViewArray(webview::webview* view, std::string prefix, std::array<int, N>* array) {
    for (int i = 0; i < array->size(); i++)
        toView(view, prefix + std::to_string(i), array->data() + i);
}

template <std::size_t N>
void toSequencerArray(webview::webview* view, std::string prefix, std::array<int, N>* array) {
    for (int i = 0; i < array->size(); i++)
        toSequencer(view, prefix + std::to_string(i), array->data() + i);
}

int main(int argc, char* argv[]) { // TODO WinMain, see webview README
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    groovebox::Input input {};
    groovebox::Sequencer sequencer {};
    groovebox::Sampler sampler {};
    groovebox::EnferUI enferUI(root);

    assert(sequencer.voiceOutCount == sampler.getNumInputs());
    sequencer.init();
    sampler.init(SAMPLE_RATE);
    sampler.buildUserInterface(&enferUI);
    UserData userData { &input, &sequencer, &sampler };

    ma_device device;
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_playback);
    deviceConfig.playback.channels = sampler.getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;
    deviceConfig.pUserData = &userData;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see docs/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (root / "ui" / "index.html").string());

    toView(&view, "beat", &sequencer.beat);
    toView(&view, "bpm", &sequencer.bpm);
    toView(&view, "play", &sequencer.playing);
    toView(&view, "arm", &sequencer.armed);
    toView(&view, "step", &sequencer.beat); // TODO delete
    toView(&view, "key", &sequencer.activeKey);
    toView(&view, "track", &sequencer.activeTrack);
    toView(&view, "type", &sequencer.activeTrackType);
    toView(&view, "page", &sequencer.activePage);
    toView(&view, "length", &sequencer.activeLength);
    toView(&view, "sounds", &sequencer.activeSounds);
    toView(&view, "root", &sequencer.root);
    toView(&view, "scale", &sequencer.scale);
    toView(&view, "octave", &sequencer.activeOctave);
    toViewArray(&view, "hit:", &sequencer.activeHits);
    toSequencer(&view, "play", &input.play);
    toSequencer(&view, "arm", &input.arm);
    toSequencer(&view, "bpm", &input.bpm);
    toSequencerArray(&view, "key:", &input.key);
    toSequencerArray(&view, "step:", &input.step);
    toSequencerArray(&view, "length:", &input.length);
    toSequencerArray(&view, "track:", &input.track);
    toSequencerArray(&view, "type:", &input.trackType);
    toSequencerArray(&view, "sounds:", &input.sounds);
    toSequencerArray(&view, "root:", &input.root);
    toSequencerArray(&view, "scale:", &input.scale);
    toSequencerArray(&view, "octave:", &input.octave);

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
    return 0;
}
