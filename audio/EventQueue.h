#include "choc/containers/choc_SingleReaderSingleWriterFIFO.h"

struct EventQueue {
    EventQueue() : handlers(), events() {
        events.reset(8); // max queue size
    }

    void on(std::string method, std::function<void(int)> callback) {
        handlers[method] = callback;
    }

    void push(std::string method, int value) {
        auto f = handlers.find(method);
        if (f != handlers.end())
            events.push({ f->second, value });
    }

    void evaluate() {
        std::pair<std::function<void(int)>, int> pair;
        while(events.pop(pair))
            pair.first(pair.second);
    }

  private:
    std::map<std::string, std::function<void(int)>> handlers;
    choc::fifo::SingleReaderSingleWriterFIFO<std::pair<std::function<void(int)>, int>> events;
};
