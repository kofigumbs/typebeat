extern crate anyhow;

use anyhow::Result;

fn main() -> Result<()> {
    // if !vendor/faust/build/bin/faust {
    //     git submodule update --init --recursive
    //     make -C vendor/faust
    // }
    // for dsp in audio/effects/*.dsp {
    //     let output_path = env::var_os("OUT_DIR");
    //     output_path.push(dsp);
    //     let output = Command::new("vendor/faust/build/bin/faust")
    //        .arg("-I")
    //        .arg("vendor/faust/libraries")
    //        .arg("-lang")
    //        .arg("rust")
    //        .arg("-o")
    //        .arg(output_path.as_str())
    //        .output()?;
    //     println!("cargo:rerun-if-changed={}", dsp);
    // }
    Ok(())
}
