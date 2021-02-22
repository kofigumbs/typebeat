#include <functional>

struct EventQueue {
    virtual void push(std::string method, int value) = 0;
};

void run(std::filesystem::path root, char* captureDeviceName, char* playbackDeviceName, std::function<void(EventQueue*)> view);
