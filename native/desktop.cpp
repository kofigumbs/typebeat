#include <cassert>
#include <filesystem>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_DECODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"
#include "SOUL/include/soul/soul_patch.h"

struct GROOVEBOX {
  int beat;
  soul::patch::PatchPlayer::Ptr player;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
  auto groovebox = (GROOVEBOX (*)) device->pUserData;
  float deinterleavedInput[device->capture.channels][frameCount],
        deinterleavedOutput[device->playback.channels][frameCount],
        *inputChannels[device->capture.channels],
        *outputChannels[device->playback.channels];

  for (int channel = 0; channel < device->capture.channels; channel++) {
    for (int frame = 0; frame < frameCount; frame++)
      deinterleavedInput[channel][frame] = ((float*) input)[channel + frame*device->capture.channels];
    inputChannels[channel] = ((float*) deinterleavedInput) + channel*frameCount;
  }

  for (int channel = 0; channel < device->playback.channels; channel++)
    outputChannels[channel] = ((float*) deinterleavedOutput) + channel*frameCount;

  const int maxMidi = 64;
  soul::MIDIEvent incomingMIDI[maxMidi], outgoingMIDI[maxMidi];
  soul::patch::PatchPlayer::RenderContext context;
  context.incomingMIDI = incomingMIDI;
  context.outgoingMIDI = outgoingMIDI;
  context.numMIDIMessagesIn = 0;
  context.numMIDIMessagesOut = 0;
  context.maximumMIDIMessagesOut = maxMidi;
  context.numFrames = frameCount;
  context.numInputChannels = device->capture.channels;
  context.numOutputChannels = device->playback.channels;
  context.inputChannels = (const float* const*) inputChannels;
  context.outputChannels = (float* const*) outputChannels;
  assert(groovebox->player->render(context) == soul::patch::PatchPlayer::RenderResult::ok);

  for (int channel = 0; channel < device->playback.channels; channel++)
    for (int frame = 0; frame < frameCount; frame++)
      ((float*) output)[channel + frame*device->playback.channels] = deinterleavedOutput[channel][frame];
}

#ifdef WIN32
int WINAPI WinMain(HINSTANCE hInt, HINSTANCE hPrevInst, LPSTR lpCmdLine, int nCmdShow) {
#else
int main() {
#endif
  ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
  deviceConfig.periodSizeInFrames = 64;
  deviceConfig.capture.channels = 2;
  deviceConfig.capture.format = ma_format_f32;
  deviceConfig.playback.channels = 2;
  deviceConfig.playback.format = ma_format_f32;
  deviceConfig.dataCallback = callback;

  ma_device device;
  assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

  auto cwd = std::filesystem::current_path();
  soul::patch::SOULPatchLibrary library((cwd / soul::patch::SOULPatchLibrary::getLibraryFileName()).c_str());
  soul::patch::PatchInstance::Ptr patch = library.createPatchFromFileBundle("dsp/groovebox.soulpatch");
  soul::patch::PatchPlayerConfiguration playerConfig;
  playerConfig.sampleRate = device.sampleRate;
  playerConfig.maxFramesPerBlock = deviceConfig.periodSizeInFrames;
  auto player = soul::patch::PatchPlayer::Ptr(
    patch->compileNewPlayer(playerConfig, NULL, NULL, NULL, NULL)
  );

  GROOVEBOX groovebox;
  groovebox.player = player;
  device.pUserData = &groovebox;

  webview::webview view(true, nullptr);
  view.set_size(780, 300, WEBVIEW_HINT_MIN);
  view.set_size(780, 300, WEBVIEW_HINT_NONE);
  view.navigate("file://" + (cwd / "web" / "index.html").string());
  view.bind("groovebox", [groovebox](std::string s) -> std::string {
    return std::string("{") +
      "\"beat\":" + std::to_string(groovebox.beat) +
    "}";
  });

#ifdef WEBVIEW_COCOA
  auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 251/255.0, 241/255.0, 199/255.0, 1.0); // see notes/frameless.md
  auto window = (id) view.window();
  objc_msgSend(window, sel_registerName("setBackgroundColor:"), light);
  objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
  objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
  objc_msgSend(window, sel_registerName("center"));
#endif

  ma_device_start(&device);
  view.run();
  ma_device_uninit(&device);
  return 0;
}
