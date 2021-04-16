#include <functional>

struct EventHandler {
    virtual void onSend(const std::string&, int) = 0;
    virtual int onReceive(const std::string&) = 0;
};

void run(std::filesystem::path, char*, char*, int, std::function<void(EventHandler*)>);
