#include <filesystem>
#include "webview/webview.h"
#include "../audio/lib.h"
#include "../build/Ui.h"

int main(int argc, char* argv[]) {
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    char* captureDeviceName = argc < 2 ? nullptr : argv[1];
    char* playbackDeviceName = argc < 3 ? captureDeviceName : argv[2];
    run(root, captureDeviceName, playbackDeviceName, [root](EventQueue* eventQueue) {
        webview::webview view(true, nullptr);
        view.set_size(960, 540, WEBVIEW_HINT_MIN);
        view.set_size(960, 540, WEBVIEW_HINT_NONE);
        view.navigate(uiHtml);
        view.bind("$push", [eventQueue](std::string data) -> std::string {
            eventQueue->push(
                data.substr(data.find_first_of("\"") + 1, data.find_last_of("\"") - 2),
                std::stoi(data.substr(data.find_last_of(",") + 1))
            );
            return "";
        });
        view.run();
    });
}
