#include <filesystem>
#include "webview/webview.h"
#include "../audio/lib.h"

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
        view.navigate("file://" + (root / "ui" / "index.html").string());
        view.bind("$push", [eventQueue](std::string data) -> std::string {
            eventQueue->push(
                data.substr(data.find_first_of("\"") + 1, data.find_last_of("\"") - 2),
                std::stoi(data.substr(data.find_last_of(",") + 1))
            );
            return "";
        });

#ifdef WEBVIEW_COCOA
        // see docs/frameless.md
        auto window = (id) view.window();
        auto background = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 60/255.0, 56/255.0, 54/255.0, 1.0);
        objc_msgSend(window, sel_registerName("setBackgroundColor:"), background);
        objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
        objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
        view.set_size(960, 540 + 22, WEBVIEW_HINT_NONE);

        objc_msgSend(window, sel_registerName("center"));
        objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif

        view.run();
    });
}
