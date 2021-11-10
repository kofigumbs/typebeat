use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let out = std::env::var("OUT_DIR")?;
    for path in Path::new("src/effects")
        .read_dir()?
        .map(|entry| entry.expect("DirEntry").path())
        .filter(|path| path.extension() == Some(OsStr::new("dsp")))
    {
        let stem = path.file_stem().expect("stem");
        let rust = Path::new(&out).join(stem).with_extension("rs");
        let _ = std::fs::remove_file(&rust);
        let mut command = Command::new("faust");
        command.args(&["-lang", "rust", "-json", "--output-dir", &out]);
        command.arg("--class-name").arg(&stem).arg(&path);
        let dsp = String::from_utf8(command.output()?.stdout)?;
        let ident = format!("pub struct {} {{", stem.to_string_lossy());
        let with_derive = format!("#[derive(Clone, default_boxed::DefaultBoxed)]\n{}", ident);
        std::fs::write(&rust, dsp.replace(&ident, &with_derive))?;
        println!("cargo:rerun-if-changed={}", path.display());
    }
    #[cfg(not(target_arch = "wasm32"))]
    tauri_build::build();
    Ok(())
}
