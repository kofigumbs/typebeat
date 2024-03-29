fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        println!("cargo:rustc-link-lib=framework=ColorSync");
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(error) = tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(
        tauri_build::WindowsAttributes::new().window_icon_path("assets/icons/icon.ico"),
    )) {
        panic!("error found during tauri-build: {}", error);
    }
}
