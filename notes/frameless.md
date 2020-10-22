macOS has an API for transparent titlebars, but it seems it breaks things for full-window webviews.
into the window... this seems like it should work but doesn't:

<https://github.com/r0x0r/pywebview/issues/169>

Here's the code I was desparately trying to make work (it seems like a shame to throw it away):

```cpp
auto contentView = (id) objc_msgSend(window, sel_registerName("contentView"));
objc_msgSend(contentView, sel_registerName("setMouseDownCanMoveWindow:"), 1); // DOES NOT EXIST, requires WKWebView subclass
objc_msgSend(window, sel_registerName("setStyleMask:"),
    1 |       // titled
    2 |       // closable
    4 |       // miniaturizable
    8 |       // resizable
    1 << 15); // fullsize (ask webview to cover space beneath titlebar)
objc_msgSend(window, sel_registerName("setMovableByWindowBackground:"), 1);
```

So if you're constrained to C++, the best you can do is copy the background color in your HTML and set it on your NSWindow's.
And to keep things symmetrical, add a `body { margin-bottom: 22px }` style.
