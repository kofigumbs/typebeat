#include <filesystem>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_DECODING
#define MA_NO_GENERATION

#include "faust/dsp/dsp.h"
#include "faust/dsp/cpp-dsp-adapter.h"
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"

#define GROOVEBOX_MAX_MIDI 64

struct UserData {
    dsp* mydsp;
    // choc::fifo::SingleReaderSingleWriterFIFO<soul::MIDIEvent> midiIn;
    // choc::fifo::SingleReaderSingleWriterFIFO<soul::MIDIEvent> midiOut;
};

// void pushMidi(std::string context, choc::fifo::SingleReaderSingleWriterFIFO<soul::MIDIEvent>* queue, soul::MIDIEvent event) {
//     if (!queue->push(event))
//         printf("dropped MIDI %s [%hhu, %hhu, %hhu]\n", context.c_str(), event.message.data[0], event.message.data[1], event.message.data[2]);
// }

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
    // soul::MIDIEvent incomingMIDI[GROOVEBOX_MAX_MIDI], outgoingMIDI[GROOVEBOX_MAX_MIDI];
    // soul::MIDIEvent event;
    // int numMIDIMessagesIn = 0;
    // while (userData->midiIn.pop(event))
    //     incomingMIDI[numMIDIMessagesIn++] = event;

    // render audio context
    userData->mydsp->compute(frameCount, inputChannels, outputChannels);

    // de-queue outgoing MIDI
    // for (int i = 0; i < context.numMIDIMessagesOut; i++)
    //     pushMidi("output", &userData->midiOut, outgoingMIDI[i]);

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

    auto path = std::filesystem::absolute(std::filesystem::path(argv[0]))
        .parent_path() // build directory
        .parent_path(); // project directory
    webview::webview view(true, nullptr);
    view.set_size(900, 320, WEBVIEW_HINT_MIN);
    view.set_size(900, 320 + 22 /* see notes/frameless.md */, WEBVIEW_HINT_NONE);
    view.navigate("file://" + (path / "web" / "index.html").string());

    UserData userData;
    userData.mydsp = mydsp;
    // userData.midiIn.reset(GROOVEBOX_MAX_MIDI);
    // userData.midiOut.reset(GROOVEBOX_MAX_MIDI);
    device.pUserData = &userData;

    // view.bind("midiIn", [&userData](std::string midiIn) -> std::string {
    //     pushMidi("input", &userData.midiIn, soul::MIDIEvent::fromPackedMIDIData(0, std::stoi(midiIn.substr(1))));
    //     return "";
    // });

    // view.bind("midiOut", [&userData](std::string midiIn) -> std::string {
    //     std::string midiOut;
    //     soul::MIDIEvent event;
    //     while (userData.midiOut.pop(event))
    //     midiOut += "," + std::to_string(event.getPackedMIDIData());
    //     return "[" + (midiOut.empty() ? "" : midiOut.substr(1)) + "]";
    // });

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
