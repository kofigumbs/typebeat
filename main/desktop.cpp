#include <regex>
#include <filesystem>
#include <functional>

#include "faust/dsp/dsp.h"

#include "../vendor/webview/webview.h"
#include "../audio/include/Audio.h"
#include "../audio/include/Effects.h"

std::vector<std::string> getArguments(std::string json) {
    std::vector<std::string> arguments;
    std::regex stringOrInt("[a-zA-Z:0-9\\-]+");
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
int main(int argc, char* argv[]) {
#endif
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    auto insert = std::unique_ptr<dsp>(create_insert());
    Audio audio {
        root,
        getenv("TYPEBEAT_INPUT_DEVICE"),
        getenv("TYPEBEAT_OUTPUT_DEVICE"),
        getenv("TYPEBEAT_VOICES") ? std::stoi(getenv("TYPEBEAT_VOICES")) : 15,
        insert.get()
    };
    audio.start([root](Audio::EventHandler* eventHandler) {
        webview::webview view(true, nullptr);
        view.set_size(1200, 400, WEBVIEW_HINT_MIN);
        view.set_size(1200, 430, WEBVIEW_HINT_NONE);
        view.bind("$send", [eventHandler](std::string json) -> std::string {
            auto arguments = getArguments(json);
            eventHandler->onSend(arguments[0], arguments.size() == 1 ? 0 : std::stoi(arguments[1]));
            return "";
        });
        view.bind("$receive", [eventHandler](std::string json) -> std::string {
            auto arguments = getArguments(json);
            int value;
            return eventHandler->onReceive(arguments[0], value) ? std::to_string(value) : "null";
        });
#ifdef WEBVIEW_COCOA
        auto window = (id) view.window();
        objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
        objc_msgSend(window, sel_registerName("center"));
        objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif
        view.navigate("file:///" + (root / "ui" / "index.html").string());
        view.run();
    });
}
