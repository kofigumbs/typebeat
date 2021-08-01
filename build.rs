extern crate anyhow;

use std::fs::{DirEntry, File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Bus {
    inputs: usize,
    outputs: usize,
}

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
        let out = Path::new(&basename)
            .with_extension("rs")
            .to_string_lossy()
            .into_owned();
        Command::new("vendor/faust/build/bin/faust")
            .args(&["-I", "vendor/faust/libraries"])
            .args(&["-lang", "rust", "-cn", &basename])
            .args(&["-O", &out_dir, "-json", "-o", &out, &path])
            .status()?;
        let bus: Bus = serde_json::from_reader(BufReader::new(File::open(
            Path::new(&out_dir)
                .join(&basename)
                .with_extension("dsp.json"),
        )?))?;
        let impl_bus = format!(
            "impl Bus for {} {{
                const INPUTS: usize = {};
                const OUTPUTS: usize = {};
            }}",
            &basename, bus.inputs, bus.outputs
        );
        OpenOptions::new()
            .append(true)
            .open(Path::new(&out_dir).join(&out))?
            .write_all(impl_bus.as_ref())?;
        println!("cargo:rerun-if-changed={}", &path);
    }
    Ok(())
}
