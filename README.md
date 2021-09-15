<h1 align="center">
  <img src="./static/logo/screenshot.png" alt="Typebeat logo" width="200" />
</h1>

Typebeat uses the following build dependencies:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Faust](https://github.com/grame-cncm/faust/releases)
- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) (only required for web)

Run the native app locally in dev mode with [Tauri CLI](https://tauri.studio/en/docs/usage/development/integration#1-install-tauri-cli-package-as-a-dev-dependency):

```bash
src-tauri/env tauri dev # ✨ QUICK-START COMMAND 💫
```

Build the web app with:

```bash
src-wrb/env cargo build -p typebeat-web
```
