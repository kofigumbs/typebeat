#include <filesystem>
#include <regex>
#include "../vendor/webview/webview.h"
#include "../audio/audio.hpp"

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
    char* captureDeviceName = argc < 2 ? nullptr : argv[1];
    char* playbackDeviceName = argc < 3 ? captureDeviceName : argv[2];
    run(root, nullptr, nullptr, [root](EventHandler* eventHandler) {
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
            return std::to_string(eventHandler->onReceive(arguments[0]));
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
