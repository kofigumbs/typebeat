#include <array>
#include <cassert>
#include <filesystem>
#include <iostream>

#include "device.h"
#include "webview/webview.h"

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

int main(int argc, char* argv[]) {
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    char* captureDeviceName = argc < 2 ? nullptr : argv[1];
    char* playbackDeviceName = argc < 3 ? captureDeviceName : argv[2];
    run(root, captureDeviceName, playbackDeviceName, [root](groovebox::Sequencer* sequencer, groovebox::Input* input) {
        webview::webview view(true, nullptr);
        view.set_size(900, 320, WEBVIEW_HINT_MIN);
        view.set_size(900, 320 + 22 /* see docs/frameless.md */, WEBVIEW_HINT_NONE);
        view.navigate("file://" + (root / "ui" / "index.html").string());

        toUi(&view, "beat", &sequencer->beat);
        toUi(&view, "page", &sequencer->page);
        syncUi(&view, "tempo", &sequencer->active.tempo, &input->tempo);
        syncUi(&view, "play", &sequencer->active.play, &input->play);
        syncUi(&view, "record", &sequencer->active.record, &input->record);
        syncUi(&view, "clock", &sequencer->active.clock, &input->clock);
        syncUi(&view, "track", &sequencer->active.track, &input->track);
        syncUi(&view, "source", &sequencer->active.source, &input->source);
        syncUi(&view, "polyphonic", &sequencer->active.polyphonic, &input->polyphonic);
        syncUi(&view, "length", &sequencer->active.length, &input->length);
        syncUi(&view, "sounds", &sequencer->active.sounds, &input->sounds);
        syncUi(&view, "root", &sequencer->active.root, &input->root);
        syncUi(&view, "scale", &sequencer->active.scale, &input->scale);
        syncUi(&view, "octave", &sequencer->active.octave, &input->octave);
        syncUi(&view, "volume", &sequencer->active.volume, &input->volume);
        syncUi(&view, "pan", &sequencer->active.pan, &input->pan);
        syncUi(&view, "filter", &sequencer->active.filter, &input->filter);
        syncUi(&view, "resonance", &sequencer->active.resonance, &input->resonance);
        syncUi(&view, "delay", &sequencer->active.delay, &input->delay);
        syncUi(&view, "reverb", &sequencer->active.reverb, &input->reverb);
        for (int i = 0; i < sequencer->active.keys.size(); i++)
            syncUi(&view, "key:" + std::to_string(i), sequencer->active.keys.data() + i, input->keys.data() + i);
        for (int i = 0; i < sequencer->active.steps.size(); i++)
            syncUi(&view, "step:" + std::to_string(i), sequencer->active.steps.data() + i, input->steps.data() + i);
        for (int i = 0; i < sequencer->active.mutes.size(); i++)
            syncUi(&view, "mute:" + std::to_string(i), sequencer->active.mutes.data() + i, input->mutes.data() + i);

#ifdef WEBVIEW_COCOA
        auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 251/255.0, 241/255.0, 199/255.0, 1.0); // see docs/frameless.md
        auto window = (id) view.window();
        objc_msgSend(window, sel_registerName("setBackgroundColor:"), light);
        objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
        objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
        objc_msgSend(window, sel_registerName("center"));
        objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif

        view.run();
    });
}
