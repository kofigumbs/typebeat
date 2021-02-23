#include <filesystem>
#include "../vendor/webview/webview.h"
#include "../audio/audio.h"

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
    run(root, nullptr, nullptr, [root](EventQueue* eventQueue) {
        webview::webview view(true, nullptr);
        view.set_size(960, 540, WEBVIEW_HINT_MIN);
        view.set_size(960, 540, WEBVIEW_HINT_NONE);
        view.bind("$push", [eventQueue](std::string data) -> std::string {
            eventQueue->push(
                data.substr(data.find_first_of("\"") + 1, data.find_last_of("\"") - 2),
                std::stoi(data.substr(data.find_last_of(",") + 1))
            );
            return "";
        });
        view.navigate("file:///" + (root / "ui" / "index.html").string());
        view.run();
    });
}
