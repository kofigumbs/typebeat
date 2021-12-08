fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        println!("cargo:rustc-link-lib=framework=ColorSync");
    }
    #[cfg(not(target_arch = "wasm32"))]
    tauri_build::build();
}
