#include <array>
#include <cassert>
#include <filesystem>
#include <iostream>
#include <map>

#include "../audio/lib.h"
#include "webview/webview.h"

int main(int argc, char* argv[]) {
    auto root = std::filesystem::canonical(argv[0])
        .parent_path() // build directory
        .parent_path(); // project directory
    char* captureDeviceName = argc < 2 ? nullptr : argv[1];
    char* playbackDeviceName = argc < 3 ? captureDeviceName : argv[2];
    run(root, captureDeviceName, playbackDeviceName, [root](EventQueue* eventQueue) {
        webview::webview view(true, nullptr);
        view.set_size(900, 320, WEBVIEW_HINT_MIN);
        view.set_size(900, 320 + 22 /* see docs/frameless.md */, WEBVIEW_HINT_NONE);
        view.navigate("file://" + (root / "ui" / "index.html").string());
        view.bind("fromUi", [eventQueue](std::string data) -> std::string {
            eventQueue->push(
                data.substr(data.find_first_of("\"") + 1, data.find_last_of("\"") - 2),
                std::stoi(data.substr(data.find_last_of(",") + 1))
            );
            return "";
        });

#ifdef WEBVIEW_COCOA
        auto light = objc_msgSend((id) objc_getClass("NSColor"), sel_registerName("colorWithRed:green:blue:alpha:"), 60/255.0, 56/255.0, 54/255.0, 1.0); // see docs/frameless.md
        auto window = (id) view.window();
        objc_msgSend(window, sel_registerName("setBackgroundColor:"), light);
        objc_msgSend(window, sel_registerName("setTitlebarAppearsTransparent:"), 1);
        objc_msgSend(window, sel_registerName("setHasShadow:"), 1);
        objc_msgSend(window, sel_registerName("center"));
        objc_msgSend(window, sel_registerName("makeFirstResponder:"), objc_msgSend(window, sel_registerName("contentView")));
#endif

        view.run();
    });
}
