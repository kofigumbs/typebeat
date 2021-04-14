#include <array>
#include <iostream>
#include <cassert>
#include <filesystem>
#include <unordered_map>

#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "../vendor/miniaudio/miniaudio.h"

#define SAMPLE_RATE 44100
#include "faust/dsp/dsp.h"
#include "faust/dsp/poly-dsp.h"
#include "faust/gui/DecoratorUI.h"
#include "faust/gui/Soundfile.h"

#include "../vendor/choc/containers/choc_SingleReaderSingleWriterFIFO.h"

Soundfile* defaultsound;
std::list<GUI*> GUI::fGuiList;
#include "../build/Insert.h"

#include "./audio.hpp"
#include "./EntryMap.hpp"
#include "./Transport.hpp"
#include "./Track.hpp"
#include "./Controller.hpp"
#include "./SampleFiles.hpp"

struct UserData {
    Controller* controller;
    mydsp_poly* dsp;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    float l, r;
    float* o[2]{ &l, &r };
    for (int frame = 0; frame < frameCount; frame++) {
        auto i = (float*) input + frame*device->capture.channels;
        userData->controller->advance();
        userData->dsp->compute(1, &i, o); // only one frame for sample-accurate controls
        ((float*) output)[frame*device->playback.channels] = l;
        ((float*) output)[frame*device->playback.channels + 1] = r;
    }
}

void run(std::filesystem::path root, char* inputDeviceName, char* outputDeviceName, std::function<void(EventHandler*)> view) {
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

    auto insert = Insert();
    auto entryMap = EntryMap();
    auto defaultSamples = std::make_unique<SampleFiles>(root / "samples");
    insert.buildUserInterface(&entryMap);

    auto dsp = std::make_unique<mydsp_poly>(&insert, 15, false, false);
    auto controller = std::make_unique<Controller>(dsp.get(), entryMap);
    dsp->init(SAMPLE_RATE);
    dsp->buildUserInterface(defaultSamples.get());
    assert(dsp->getNumInputs() == 1);
    assert(dsp->getNumOutputs() == 2);
    UserData userData { controller.get(), dsp.get() };

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
    deviceConfig.pUserData = &userData;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    assert(ma_device_start(&device) == MA_SUCCESS);
    view(controller.get());
    ma_device_uninit(&device);
    ma_context_uninit(&context);
}
