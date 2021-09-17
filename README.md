<h1 align="center">
  <img src="./static/logo/screenshot.png" alt="Typebeat logo" width="200" />
</h1>

Typebeat uses the following build dependencies:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Faust](https://github.com/grame-cncm/faust/releases)

Build and run the native app locally:

```bash
src-tauri/env cargo run -p typebeat # ✨ QUICK-START COMMAND 💫
```

Build the web app (requires [Emscripten](https://emscripten.org/docs/getting_started/downloads.html)):

```bash
src-web/env cargo build -p typebeat-web --release
```
