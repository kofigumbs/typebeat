<h1 align="center">
  <img src="./static/ui/logo/screenshot.png" alt="Typebeat logo" width="200" />
</h1>

```
cargo run
```

## Web build (on macOS)

1. `brew install llvm emscripten`
2. ```
   LLVM_CONFIG_PATH="/usr/local/opt/llvm/bin/llvm-config" \
   BINDGEN_EXTRA_CLANG_ARGS="--target=wasm32-unknown-emscripten --sysroot=/usr/local/opt/emscripten/libexec/cache/sysroot -fvisibility=default" \
   cargo build --target wasm32-unknown-emscripten --bin typebeat-lite
   ```
