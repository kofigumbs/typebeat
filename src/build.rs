use std::error::Error;
use std::fs::DirEntry;
use std::path::Path;
use std::process::Command;

fn dsp_file(entry: DirEntry) -> Option<(String, String)> {
    match entry.path().extension()?.to_str()? {
        "dsp" => Some((
            entry.path().to_str()?.to_owned(),
            entry.path().file_stem()?.to_str()?.to_owned(),
        )),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for (path, basename) in Path::new("src/effects")
        .read_dir()?
        .flat_map(|x| dsp_file(x.ok()?))
    {
        let out = Path::new(&std::env::var("OUT_DIR")?)
            .join(&basename)
            .with_extension("rs");
        let _ = std::fs::remove_file(&out);
        let dsp = Command::new("faust")
            .args(&["-lang", "rust", "-cn", &basename, &path])
            .output()?
            .stdout;
        let ident = format!("pub struct {} {{", basename);
        let with_derive = format!("#[derive(Clone, default_boxed::DefaultBoxed)]\n{}", ident);
        std::fs::write(&out, String::from_utf8(dsp)?.replace(&ident, &with_derive))?;
        println!("cargo:rerun-if-changed={}", &path);
    }
    Ok(())
}
