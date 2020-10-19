#define MINIAUDIO_IMPLEMENTATION
#include "miniaudio/miniaudio.h"
#include "webview/webview.h"

#include <stdio.h>

#ifdef __EMSCRIPTEN__
void main_loop__em()
{
}
#endif

void data_callback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount) {
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
  deviceConfig.capture.pDeviceID  = NULL;
  deviceConfig.capture.format     = ma_format_s16;
  deviceConfig.capture.channels   = 2;
  deviceConfig.capture.shareMode  = ma_share_mode_shared;
  deviceConfig.playback.pDeviceID = NULL;
  deviceConfig.playback.format    = ma_format_s16;
  deviceConfig.playback.channels  = 2;
  deviceConfig.dataCallback       = data_callback;
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
  ma_device_start(&device);
  webview::webview w(true, nullptr);
  w.set_title("Groovebox");
  w.set_size(480, 320, WEBVIEW_HINT_NONE);
  w.navigate("https://en.m.wikipedia.org/wiki/Main_Page");
  w.run();
#endif

  ma_device_uninit(&device);
  return 0;
}
