#include <functional>

struct EventHandler {
    virtual void onSend(std::string name, int value) = 0;
    virtual int onReceive(std::string name) = 0;
};

void run(std::filesystem::path root, char* captureDeviceName, char* playbackDeviceName, std::function<void(EventHandler*)> view);
