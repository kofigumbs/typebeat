#include <functional>

struct EventHandler {
    virtual void onSend(std::string, int) = 0;
    virtual int onReceive(std::string) = 0;
};

void run(std::filesystem::path, char*, char*, std::function<void(EventHandler*)>);
