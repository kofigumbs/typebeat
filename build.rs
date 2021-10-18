use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// Like an if expression, but terser
fn if_<T>(cond: bool, when_true: T, when_false: T) -> T {
    return if cond { when_true } else { when_false };
}

/// Convert lowerCamelCase to snake_case
fn snake(ident: &str) -> String {
    ident
        .chars()
        .map(|c| if_(c.is_uppercase(), format!("_{}", c.to_lowercase()), c.into()))
        .collect()
}

#[derive(Deserialize)]
struct FaustParam {
    #[serde(rename = "type")]
    type_: String,
    label: String,
    init: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,
}

#[derive(Deserialize)]
struct Ui {
    items: Vec<FaustParam>,
}

#[derive(Deserialize)]
struct Description {
    ui: (Ui,),
}

#[derive(Clone, Copy)]
enum Type {
    Bool(bool),
    I32(i32),
    F32(f32),
    Usize(usize),
}

#[derive(Clone)]
struct Param {
    label: String,
    type_: Type,
    min_: Option<Type>,
    max_: Option<Type>,
    step_: Option<Type>,
    array_: Option<usize>,
    ephemeral_: bool,
    dsp_id: Option<(String, usize)>,
}

impl Param {
    fn new(label: &'static str, type_: Type) -> Self {
        Self {
            label: label.to_owned(),
            type_,
            min_: None,
            max_: None,
            step_: None,
            array_: None,
            ephemeral_: false,
            dsp_id: None,
        }
    }

    fn step(mut self, step: i32) -> Self {
        self.step_ = Some(Type::I32(step));
        self
    }

    fn max(mut self, max: i32) -> Self {
        self.max_ = Some(Type::I32(max));
        self
    }

    fn clamp(mut self, min: i32, max: i32) -> Self {
        self.min_ = Some(Type::I32(min));
        self.max_ = Some(Type::I32(max));
        self
    }

    fn group(mut self, length: usize) -> Self {
        self.array_ = Some(length);
        self
    }

    fn ephemeral(mut self) -> Self {
        self.ephemeral_ = true;
        self
    }

    fn rust_field(&self) -> String {
        let param = match self.type_ {
            Type::Bool(_) => "Param<bool>",
            Type::I32(_) => "Param<i32>",
            Type::F32(_) => "Param<f32>",
            Type::Usize(_) => "Param<usize>",
        };
        let type_ = match self.array_ {
            None => param.to_owned(),
            Some(length) => format!("[{}; {}]", param, length),
        };
        format!("pub {}: {},\n", snake(&self.label), type_)
    }

    fn rust_default(&self) -> String {
        let value = |type_| match type_ {
            Type::Bool(b) => b.to_string(),
            Type::I32(i) => i.to_string(),
            Type::F32(f) => f.to_string() + "_f32",
            Type::Usize(u) => u.to_string(),
        };
        let maybe_value = |type_| match type_ {
            None => "None".to_owned(),
            Some(type_) => format!("Some({})", value(type_)),
        };
        let param = format!("Param::new({}", value(self.type_))
            + &format!(", {}", maybe_value(self.min_))
            + &format!(", {}", maybe_value(self.max_))
            + &format!(", {}", maybe_value(self.step_))
            + &format!(", {}", self.ephemeral_)
            + &format!(", {:?}),\n", self.dsp_id);
        match self.array_ {
            None => format!("{}: {}", snake(&self.label), param),
            Some(length) => format!("{}: [\n{}],\n", snake(&self.label), param.repeat(length)),
        }
    }

    fn rust_visit_call(&self, label_suffix: &str, field_suffix: &str) -> String {
        format!("visitor.call(\"{}{}\", ", self.label, label_suffix)
            + &format!("&self.{}{});\n", snake(&self.label), field_suffix)
    }

    fn rust_visit(&self) -> String {
        match self.array_ {
            None => self.rust_visit_call("", ""),
            Some(length) => (0..length)
                .map(|i| self.rust_visit_call(&i.to_string(), &format!("[{}]", i)))
                .collect(),
        }
    }
}

fn generate_rust(params: &[Param]) -> String {
    let mut fields = String::new();
    let mut defaults = String::new();
    let mut visits = String::new();
    for param in params {
        fields += &param.rust_field();
        defaults += &param.rust_default();
        visits += &param.rust_visit();
    }
    let mut s = String::new();
    s += "#[derive(Clone)]\n";
    s += &format!("pub struct State {{\n{}}}\n\n", fields);
    s += "impl Default for State {\n";
    s += &format!("fn default() -> Self {{\nSelf {{\n{}}}\n}}\n}}\n", defaults);
    s += "impl IsState for State {\n";
    s += "fn visit_params<T: Visitor>(&self, visitor: &mut T) {\n";
    s += &format!("{}}}\n}}\n", visits);
    s
}

fn compile_faust(out: &Path, path: &Path, stem: &OsStr) -> Result<(), Box<dyn Error>> {
    let rust = out.join(stem).with_extension("rs");
    let _ = std::fs::remove_file(&rust);
    let mut command = Command::new("faust");
    command.args(&["-lang", "rust", "-json"]);
    command.arg("--output-dir").arg(&out);
    command.arg("--class-name").arg(&stem).arg(&path);
    let dsp = String::from_utf8(command.output()?.stdout)?;
    let ident = format!("pub struct {} {{", stem.to_string_lossy());
    let with_derive = format!("#[derive(Clone, default_boxed::DefaultBoxed)]\n{}", ident);
    std::fs::write(&rust, dsp.replace(&ident, &with_derive))?;
    println!("cargo:rerun-if-changed={}", path.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out = PathBuf::from(std::env::var("OUT_DIR")?);
    let effects = Path::new("src/effects")
        .read_dir()?
        .map(|entry| entry.expect("DirEntry").path())
        .filter(|path| path.extension() == Some(OsStr::new("dsp")))
        .collect::<Vec<_>>();
    let mut song_params = vec![
        Param::new("activeTrackId", Type::Usize(0)).max(14),
        Param::new("playing", Type::Bool(false)),
        Param::new("recording", Type::Bool(false)),
        Param::new("tempo", Type::Usize(120)).max(999).step(10),
        Param::new("root", Type::I32(0)).clamp(-12, 12).step(7),
        Param::new("scale", Type::Usize(0)).max(4),
    ];
    let mut track_params = vec![
        Param::new("activeKey", Type::Usize(12)).max(14),
        Param::new("canClear", Type::Bool(false)).ephemeral(),
        Param::new("muted", Type::Bool(false)),
        Param::new("useKey", Type::Bool(true)),
        Param::new("octave", Type::Usize(4)).clamp(2, 8),
        Param::new("resolution", Type::Usize(16)).clamp(1, 512),
        Param::new("length", Type::Usize(512)).clamp(512, 512 * 8),
        Param::new("bars", Type::Usize(0)).ephemeral(),
        Param::new("viewStart", Type::Usize(0)).ephemeral(),
        Param::new("pageStart", Type::Usize(0)).ephemeral(),
        Param::new("recent", Type::Usize(0)).ephemeral(),
        Param::new("view", Type::Usize(0)).group(4).ephemeral(),
        Param::new("waveform", Type::I32(0)).group(24).ephemeral(),
    ];
    for path in effects.iter() {
        let stem = path.file_stem().expect("stem");
        compile_faust(&out, path, stem)?;
        let json = std::fs::read(out.join(stem).with_extension("dsp.json"))?;
        let description = serde_json::from_slice::<Description>(&json)?;
        let params = if_(stem == "insert", &mut track_params, &mut song_params);
        for (i, faust_param) in description.ui.0.items.into_iter().enumerate() {
            if faust_param.type_ == "nentry" {
                params.push(Param {
                    label: faust_param.label,
                    type_: Type::F32(faust_param.init.expect("init")),
                    step_: faust_param.step.map(Type::F32),
                    min_: faust_param.min.map(Type::F32),
                    max_: faust_param.max.map(Type::F32),
                    array_: None,
                    ephemeral_: false,
                    dsp_id: Some((stem.to_string_lossy().into_owned(), i)),
                });
            }
        }
    }
    std::fs::write(&out.join("song.rs"), generate_rust(&song_params))?;
    std::fs::write(&out.join("track.rs"), generate_rust(&track_params))?;
    #[cfg(not(feature = "netlify"))]
    tauri_build::build();
    Ok(())
}
