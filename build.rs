fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    tauri_build::build();
}
