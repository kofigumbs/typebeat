#include <array>
#include <iostream>
#include <cassert>
#include <filesystem>
#include <set>
#include <unordered_map>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "../vendor/miniaudio/miniaudio.h"

#define SAMPLE_RATE 44100
#include "faust/dsp/dsp.h"
#include "faust/gui/meta.h"

#include "../vendor/choc/containers/choc_SingleReaderSingleWriterFIFO.h"

#include "./audio.h"
#include "./Destinations.h"
#include "./Voice.h"
#include "./Media.h"
#include "./Controller.h"

#include "./faust/UI.h"
#include "./faust/one_sample_dsp.h"
#include "../build/Effects.h"

struct UserData {
    Controller* controller;
    Effects* effects;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    for (int frame = 0; frame < frameCount; frame++) {
        userData->controller->render(((float*) input)[frame]);
        userData->effects->render((float*) userData->controller->output.data(), ((float*) output) + frame*device->playback.channels);
    }
}

void run(std::filesystem::path root, char* captureDeviceName, char* playbackDeviceName, std::function<void(EventHandler*)> view) {
    ma_context context;
    ma_device_id* captureDeviceId = nullptr;
    ma_device_id* playbackDeviceId = nullptr;
    ma_uint32 captureDeviceCount;
    ma_device_info* captureDeviceInfo;
    ma_uint32 playbackDeviceCount;
    ma_device_info* playbackDeviceInfo;
    assert(ma_context_init(NULL, 0, NULL, &context) == MA_SUCCESS);
    assert(ma_context_get_devices(&context, &playbackDeviceInfo, &playbackDeviceCount,  &captureDeviceInfo, &captureDeviceCount) == MA_SUCCESS);
    if (captureDeviceName != nullptr) {
        for (int i = 0; i < captureDeviceCount; ++i)
            if (strcmp(captureDeviceName, captureDeviceInfo[i].name) == 0)
                captureDeviceId = &captureDeviceInfo[i].id;
        assert(captureDeviceId != nullptr);
    }
    if (playbackDeviceName != nullptr) {
        for (int i = 0; i < playbackDeviceCount; ++i)
            if (strcmp(playbackDeviceName, playbackDeviceInfo[i].name) == 0)
                playbackDeviceId = &playbackDeviceInfo[i].id;
        assert(playbackDeviceId != nullptr);
    }

    auto effects = std::make_unique<Effects>();
    effects->prepare();
    auto controller = std::make_unique<Controller>(root, effects->destinations.get());
    assert(sizeof(controller->output) == effects->getNumInputs() * sizeof(float));
    UserData userData { controller.get(), effects.get() };

    ma_device device;
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = 1;
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.capture.pDeviceID = captureDeviceId;
    deviceConfig.playback.channels = effects->getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.playback.pDeviceID = playbackDeviceId;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;
    deviceConfig.pUserData = &userData;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    ma_device_start(&device);
    view(controller.get());
    ma_device_uninit(&device);
    ma_context_uninit(&context);
}
