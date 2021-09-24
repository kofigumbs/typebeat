<img align="left" src="./src/icons/icon.svg" alt="Logo" style="width: 80px; height: 80px;" width="80px" height="80px" />

# Typebeat

Building the native app locally requires the following dependencies:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Node/NPM](https://nodejs.org/)
- [Faust](https://github.com/grame-cncm/faust/releases)
- [Tauri CLI](https://github.com/tauri-apps/tauri/tree/dev/tooling/cli.rs)

```bash
cargo tauri build # ✨ QUICK-START COMMAND 💫
```

Building the web app requires additional dependencies:

- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html)
- [Netlify CLI](https://docs.netlify.com/cli/get-started/)

```bash
# Workaround for https://github.com/rust-lang/rust-bindgen/issues/1780
# Only required for building miniaudio the first time since cargo will cache the artifact
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=`emconfigure env 2> /dev/null | sed -n -e 's/^EMSCRIPTEN=//p'`/cache/sysroot"

netlify build
```
