#define MINIAUDIO_IMPLEMENTATION
#define MA_NO_ENCODING
#define MA_NO_GENERATION
#include "miniaudio/miniaudio.h"

#define SAMPLE_RATE 44100
#include "faust/dsp/one-sample-dsp.h"
#include "faust/gui/meta.h"
#include "faust/gui/UI.h"

#include "effects.h"
#include "sequencer.h"

struct UserData {
    groovebox::Input* input;
    groovebox::Sequencer* sequencer;
    groovebox::Effects* effects;
};

void callback(ma_device* device, void* output, const void* input, ma_uint32 frameCount) {
    auto userData = (UserData*) device->pUserData;
    int intControls[userData->effects->getNumIntControls()];
    float floatControls[userData->effects->getNumRealControls()];
    userData->effects->control(intControls, floatControls);
    for (int frame = 0; frame < frameCount; frame++) {
        userData->sequencer->compute(*(userData->input), ((float*) input)[frame]);
        userData->effects->compute((float*) userData->sequencer->output.data(), ((float*) output) + frame*device->playback.channels, intControls, floatControls);
    }
}

void run(std::filesystem::path root, char* captureDeviceName, char* playbackDeviceName, std::function<void(groovebox::Sequencer*, groovebox::Input*)> view) {
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

    groovebox::Input input {};
    groovebox::Sequencer sequencer {};
    groovebox::Effects effects {};

    assert(sizeof(sequencer.output) == effects.getNumInputs() * sizeof(float));
    sequencer.init(root);
    effects.init(SAMPLE_RATE);
    UserData userData { &input, &sequencer, &effects };

    ma_device device;
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_duplex);
    deviceConfig.capture.channels = 1;
    deviceConfig.capture.format = ma_format_f32;
    deviceConfig.capture.pDeviceID = captureDeviceId;
    deviceConfig.playback.channels = effects.getNumOutputs();
    deviceConfig.playback.format = ma_format_f32;
    deviceConfig.playback.pDeviceID = playbackDeviceId;
    deviceConfig.sampleRate = SAMPLE_RATE;
    deviceConfig.dataCallback = callback;
    deviceConfig.pUserData = &userData;
    assert(ma_device_init(NULL, &deviceConfig, &device) == MA_SUCCESS);

    ma_device_start(&device);
    view(&sequencer, &input);
    ma_device_uninit(&device);
    ma_context_uninit(&context);
}
