#include <cassert>
#include <filesystem>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_DECODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"
#include "SOUL/include/soul/soul_patch.h"
#include "SOUL/include/soul/3rdParty/choc/containers/choc_SingleReaderSingleWriterFIFO.h"

#define GROOVEBOX_MAX_MIDI 64

struct UserData {
    soul::patch::PatchPlayer::Ptr player;
    choc::fifo::SingleReaderSingleWriterFIFO<soul::MIDIEvent> midiIn;
    choc::fifo::SingleReaderSingleWriterFIFO<soul::MIDIEvent> midiOut;
};

void warnIfDropped(std::string context, bool ok) {
    if (!ok)
        printf("WARN: Dropped a MIDI %s event!\n", context.c_str());
}

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData (*)) device->pUserData;
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

    // queue incoming MIDI
    soul::MIDIEvent incomingMIDI[GROOVEBOX_MAX_MIDI], outgoingMIDI[GROOVEBOX_MAX_MIDI];
    soul::MIDIEvent event;
    int numMIDIMessagesIn = 0;
    while (userData->midiIn.pop(event))
        incomingMIDI[numMIDIMessagesIn++] = event;

    // render audio context
    soul::patch::PatchPlayer::RenderContext context;
    context.incomingMIDI = incomingMIDI;
    context.outgoingMIDI = outgoingMIDI;
    context.numMIDIMessagesIn = numMIDIMessagesIn;
    context.maximumMIDIMessagesOut = GROOVEBOX_MAX_MIDI;
    context.numFrames = frameCount;
    context.numInputChannels = device->capture.channels;
    context.numOutputChannels = device->playback.channels;
    context.inputChannels = (const float* const*) inputChannels;
    context.outputChannels = (float* const*) outputChannels;
    assert(userData->player->render(context) == soul::patch::PatchPlayer::RenderResult::ok);

    // de-queue outgoing MIDI
    for (int i = 0; i < context.numMIDIMessagesOut; i++)
        warnIfDropped("output", userData->midiOut.push(outgoingMIDI[i]));

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
    // setup miniaudio configuration to match that of the SOUL patch
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.periodSizeInFrames = 64;
    deviceConfig.capture.channels = 2;
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.playback.channels = 2;
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.dataCallback = callback;

    // initialize audio devive
    ma_device device;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    // compile SOUL patch
    auto path = std::filesystem::absolute(std::filesystem::path(std::string(argv[0])))
        .parent_path() // build directory
        .parent_path(); // project directory
    soul::patch::SOULPatchLibrary library((path / "build" / soul::patch::SOULPatchLibrary::getLibraryFileName()).c_str());
    soul::patch::PatchInstance::Ptr patch = library.createPatchFromFileBundle((path / "dsp" / "groovebox.soulpatch").c_str());
    soul::patch::PatchPlayerConfiguration playerConfig;
    playerConfig.sampleRate = device.sampleRate;
    playerConfig.maxFramesPerBlock = deviceConfig.periodSizeInFrames;
    auto player = soul::patch::PatchPlayer::Ptr(
        patch->compileNewPlayer(playerConfig, NULL, NULL, NULL, NULL)
    );

    // setup user data
    UserData userData;
    userData.player = player;
    userData.midiIn.reset(GROOVEBOX_MAX_MIDI);
    userData.midiOut.reset(GROOVEBOX_MAX_MIDI);
    device.pUserData = &userData;

    // setup webview
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (path / "web" / "index.html").string());

    // midi from webview to SOUL
    view.bind("midiIn", [&userData](std::string midiIn) -> std::string {
        warnIfDropped("input", userData.midiIn.push(soul::MIDIEvent::fromPackedMIDIData(0, std::stoi(midiIn.substr(1)))));
        return "";
    });

    // midi from SOUL to webview
    view.bind("midiOut", [&userData](std::string midiIn) -> std::string {
        std::string midiOut;
        soul::MIDIEvent event;
        while (userData.midiOut.pop(event))
        midiOut += "," + std::to_string(event.getPackedMIDIData());
        return "[" + (midiOut.empty() ? "" : midiOut.substr(1)) + "]";
    });

    // customize webview further for macOS
#ifdef WEBVIEW_COCOA
    auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 251/255.0, 241/255.0, 199/255.0, 1.0); // see notes/frameless.md
    auto window = (id) view.window();
    objc_msgSend(window, sel_registerName("setBackgroundColor:"), light);
    objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
    objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
    objc_msgSend(window, sel_registerName("center"));
    objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif

    // main run loop
    ma_device_start(&device);
    view.run();
    ma_device_uninit(&device);
    return 0;
}
