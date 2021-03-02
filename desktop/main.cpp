#include <filesystem>
#include "../vendor/webview/webview.h"
#include "../audio/audio.h"

std::string stringArgument(std::string s) {
    return s.substr(s.find_first_of("\"") + 1, s.find_last_of("\"") - 2);
}

#ifdef _WIN32
int CALLBACK WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
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
        view.bind("$send", [eventHandler](std::string data) -> std::string {
            eventHandler->onSend(stringArgument(data), std::stoi(data.substr(data.find_last_of(",") + 1)));
            return "";
        });
        view.bind("$receive", [eventHandler](std::string data) -> std::string {
            return std::to_string(eventHandler->onReceive(stringArgument(data)));
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
