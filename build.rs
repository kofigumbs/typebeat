extern crate anyhow;

use std::fs::DirEntry;
use std::path::Path;
use std::process::Command;

use anyhow::Result;

fn dsp_file(entry: DirEntry) -> Option<(String, String)> {
    if entry.path().extension()? != "dsp" {
        return None;
    }
    Some((
        entry.path().to_string_lossy().into_owned(),
        entry.path().file_stem()?.to_string_lossy().into_owned(),
    ))
}

fn main() -> Result<()> {
    if !Path::new("vendor/faust/build/bin/faust").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()?;
        Command::new("make")
            .args(&["-C", "vendor/faust"])
            .status()?;
    }
    let out_dir = std::env::var("OUT_DIR")?;
    let entries = Path::new("src/effects").read_dir()?;
    for (path, basename) in entries.flat_map(|x| dsp_file(x.ok()?)) {
        let out = Path::new(&out_dir)
            .join(&basename)
            .with_extension("rs")
            .to_string_lossy()
            .into_owned();
        Command::new("vendor/faust/build/bin/faust")
            .args(&["-I", "vendor/faust/libraries"])
            .args(&["-lang", "rust", "-cn", &basename, "-o", &out, &path])
            .status()?;
        println!("cargo:rerun-if-changed={}", &path);
    }
    Ok(())
}
