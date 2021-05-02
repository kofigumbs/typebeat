#include <regex>
#include <filesystem>
#include <functional>
#include <iostream>

#include "faust/dsp/dsp.h"

#include "../vendor/cpp-base64/base64.h"

#include "../vendor/webview/webview.h"

#include "../audio/include/Audio.h"
#include "../audio/include/Effects.h"

std::unique_ptr<webview::webview> view;

std::vector<std::string> getArguments(std::string json) {
    std::vector<std::string> arguments;
    std::regex stringOrInt("[a-zA-Z0-9:\\-]+");
    std::smatch match; 
    while(regex_search(json, match, stringOrInt)) {
        arguments.push_back(match.str());
        json = match.suffix();
    }
    return arguments;
}

#ifdef _WIN32
int CALLBACK WinMain(HINSTANCE, HINSTANCE, LPSTR, int) {
    int argc = __argc;
    char** argv = __argv;
#else
#include <signal.h>
void quit(int s) { if (view.get()) view->terminate(); }
int main(int argc, char* argv[]) {
    signal(SIGINT, quit);
#endif
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    auto insert = std::unique_ptr<dsp>(create_insert());
    Audio audio {
        root,
        getenv("TYPEBEAT_INPUT_DEVICE"),
        getenv("TYPEBEAT_OUTPUT_DEVICE"),
        getenv("TYPEBEAT_VOICES") ? std::stoi(getenv("TYPEBEAT_VOICES")) : 8,
        insert.get()
    };
    audio.start([root](Audio::EventHandler* eventHandler) {
        view = std::make_unique<webview::webview>(true, nullptr);
        view->set_size(1200, 400, WEBVIEW_HINT_MIN);
        view->set_size(1200, 430, WEBVIEW_HINT_NONE);
        view->bind("$send", [eventHandler](std::string json) -> std::string {
            auto arguments = getArguments(json);
            eventHandler->onSend(arguments[0], arguments.size() == 1 ? 0 : std::stoi(arguments[1]));
            return "";
        });
        view->bind("$receive", [eventHandler](std::string json) -> std::string {
            auto arguments = getArguments(json);
            int value;
            return eventHandler->onReceive(arguments[0], value) ? std::to_string(value) : "null";
        });
        view->bind("$drop", [eventHandler](std::string json) -> std::string {
            auto i = std::stoi(json.substr(1, 2));
            auto x = i < 10 ? 4 : 5;
            auto data = base64_decode(std::string_view(json.c_str() + x, json.size() - x - 2));
            eventHandler->drop(i, data.c_str());
            return "";
        });
        view->bind("$quit", [](std::string) -> std::string {
            view->terminate();
            return "";
        });
#ifdef WEBVIEW_COCOA
        auto window = (id) view->window();
        objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
        objc_msgSend(window, sel_registerName("center"));
        objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif
        view->navigate("file:///" + (root / "ui" / "index.html").string());
        view->run();
    });
}
