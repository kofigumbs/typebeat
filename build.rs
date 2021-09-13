extern crate anyhow;

use std::fs::DirEntry;
use std::path::Path;
use std::process::Command;

use anyhow::Error;

fn dsp_file(entry: DirEntry) -> Option<(String, String)> {
    match entry.path().extension()?.to_str()? {
        "dsp" => Some((
            entry.path().to_str()?.to_owned(),
            entry.path().file_stem()?.to_str()?.to_owned(),
        )),
        _ => None,
    }
}

fn main() -> Result<(), Error> {
    if !Path::new("vendor/faust/build/bin/faust").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()?;
        Command::new("make")
            .args(&["-C", "vendor/faust"])
            .status()?;
    }
    for (path, basename) in Path::new("src/effects")
        .read_dir()?
        .flat_map(|x| dsp_file(x.ok()?))
    {
        let out = Path::new(&std::env::var("OUT_DIR")?)
            .join(&basename)
            .with_extension("rs");
        let _ = std::fs::remove_file(&out);
        Command::new("vendor/faust/build/bin/faust")
            .args(&["-I", "vendor/faust/libraries"])
            .args(&["-lang", "rust", "-cn", &basename])
            .args(&["-o", &out.to_string_lossy(), &path])
            .status()?;
        println!("cargo:rerun-if-changed={}", &path);
    }
    Ok(())
}
