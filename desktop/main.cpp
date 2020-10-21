#define MINIAUDIO_IMPLEMENTATION
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"

#include <stdio.h>
#include <filesystem>
#include <iostream>

#ifdef __EMSCRIPTEN__
void main_loop__em()
{
}
#endif

volatile int beat = 0;
const static float bpm = 120;

void audioCallback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount) {
  static int beatDuration = (int) 60/bpm * pDevice->sampleRate / 2; /* EIGTH NOTES */
  static int beatProgress;
  beatProgress += frameCount;
  while (beatProgress >= beatDuration) {
    beat++;
    beatProgress -= beatDuration;
  }

  MA_ASSERT(pDevice->capture.format == pDevice->playback.format);
  MA_ASSERT(pDevice->capture.channels == pDevice->playback.channels);
  MA_COPY_MEMORY(pOutput, pInput, frameCount * ma_get_bytes_per_frame(pDevice->capture.format, pDevice->capture.channels));
}

#ifdef WIN32
int WINAPI WinMain(HINSTANCE hInt, HINSTANCE hPrevInst, LPSTR lpCmdLine, int nCmdShow) {
#else
int main() {
#endif
  ma_result result;
  ma_device_config deviceConfig;
  ma_device device;

  deviceConfig = ma_device_config_init(ma_device_type_duplex);
  deviceConfig.capture.channels = 2;
  deviceConfig.playback.channels = 2;
  deviceConfig.dataCallback = audioCallback;
  result = ma_device_init(NULL, &deviceConfig, &device);
  if (result != MA_SUCCESS) {
    printf("Error opening audio device");
    return result;
  }

#ifdef __EMSCRIPTEN__
  getchar();
  ma_device_start(&device);
  emscripten_set_main_loop(main_loop__em, 0, 1);
#else
  webview::webview view(true, nullptr);
  view.set_title("Groovebox");
  view.set_size(960, 420, WEBVIEW_HINT_NONE);
  view.navigate("file://" + (std::filesystem::current_path() / "web" / "index.html").string());
  view.bind("groovebox", [device](std::string s) -> std::string {
    return std::string("{") +
      "\"measure\": 16," +
      "\"beat\":" + std::to_string(beat) +
    "}";
  });
  device.pUserData = &view;
  ma_device_start(&device);
#ifdef WEBVIEW_COCOA
  objc_msgSend((id) view.window(), sel_registerName("center"));
  objc_msgSend((id) view.window(), sel_registerName("setHasShadow:"), 1);
#endif
  view.run();
#endif

  ma_device_uninit(&device);
  return 0;
}
