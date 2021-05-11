#include <array>
#include <cassert>
#include <filesystem>
#include <functional>
#include <iostream>
#include <sstream>
#include <unordered_map>

#define SAMPLE_RATE 44100
#include "faust/dsp/dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/MapUI.h"
#include "faust/gui/DecoratorUI.h"

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "../vendor/miniaudio/miniaudio.h"

#include "../vendor/choc/text/choc_Files.h"
#include "../vendor/choc/containers/choc_SingleReaderSingleWriterFIFO.h"

#include "./include/Audio.h"
#include "./include/Effects.h"

std::string twoDigit(int i) {
    return (i < 10 ? "0" : "") + std::to_string(i);
}

#include "./Autosave.hpp"
#include "./Entries.hpp"
#include "./Samples.hpp"
#include "./Voices.hpp"
#include "./Song.hpp"
#include "./Track.hpp"
#include "./Controller.hpp"

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    for (int frame = 0; frame < frameCount; frame++) {
        ((Controller*) device->pUserData)->run(
            ((float*) input)[frame*device->capture.channels],
            ((float*) output) + frame*device->playback.channels
        );
    }
}

void Audio::start(std::function<void(EventHandler*)> view) {
    ma_context context;
    ma_device_id* captureDeviceId = nullptr;
    ma_device_id* playbackDeviceId = nullptr;
    ma_uint32 captureDeviceCount;
    ma_device_info* captureDeviceInfo;
    ma_uint32 playbackDeviceCount;
    ma_device_info* playbackDeviceInfo;
    assert(ma_context_init(NULL, 0, NULL, &context) == MA_SUCCESS);
    assert(ma_context_get_devices(&context, &playbackDeviceInfo, &playbackDeviceCount,  &captureDeviceInfo, &captureDeviceCount) == MA_SUCCESS);
    if (inputDeviceName != nullptr) {
        for (int i = 0; i < captureDeviceCount; ++i)
            if (strcmp(inputDeviceName, captureDeviceInfo[i].name) == 0)
                captureDeviceId = &captureDeviceInfo[i].id;
        assert(captureDeviceId != nullptr);
    }
    if (outputDeviceName != nullptr) {
        for (int i = 0; i < playbackDeviceCount; ++i)
            if (strcmp(outputDeviceName, playbackDeviceInfo[i].name) == 0)
                playbackDeviceId = &playbackDeviceInfo[i].id;
        assert(playbackDeviceId != nullptr);
    }

    auto autosave = std::make_unique<Autosave>(root / ".typebeat");
    auto samples = std::make_unique<Samples>(root / "audio" / "samples");
    assert(voiceCount > 0);
    assert(samples->data.size() >= Controller::trackCount);
    auto voices = std::make_unique<Voices>(autosave.get(), voiceCount);
    auto controller = std::make_unique<Controller>(autosave.get(), voices.get(), samples.get());

    ma_device device;
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = 1;
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.capture.pDeviceID = captureDeviceId;
    deviceConfig.playback.channels = 2;
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.playback.pDeviceID = playbackDeviceId;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;
    deviceConfig.pUserData = controller.get();
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);
    assert(ma_device_start(&device) == MA_SUCCESS);
    quit = [&]() {
        ma_device_uninit(&device);
        ma_context_uninit(&context);
        autosave->write();
    };
    view(controller.get());
    quit();
};
