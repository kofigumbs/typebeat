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
    label: String,
    init: Option<i32>,
    min: Option<i32>,
    max: Option<i32>,
    step: Option<i32>,
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
    Usize(usize),
}

#[derive(Clone)]
struct Param {
    label: String,
    type_: Type,
    min_: Option<Type>,
    max_: Option<Type>,
    step_: Option<i32>,
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
        self.step_ = Some(step);
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

    fn is_button(&self) -> bool {
        self.step_.is_none() && self.dsp_id.is_some()
    }

    fn rust_button(&self) -> String {
        format!("pub const {}: usize", snake(&self.label).to_uppercase())
            + &format!("= {};\n", self.dsp_id.as_ref().expect("dsp_id").1)
    }

    fn rust_field(&self) -> String {
        let param = match self.type_ {
            Type::Bool(_) => "Param<bool>",
            Type::I32(_) => "Param<i32>",
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
            Type::Usize(u) => u.to_string(),
        };
        let maybe_value = |type_| match type_ {
            None => "None".to_owned(),
            Some(type_) => format!("Some({})", value(type_)),
        };
        let param = format!("Param::new({}", value(self.type_))
            + &format!(", {}", maybe_value(self.min_))
            + &format!(", {}", maybe_value(self.max_))
            + &format!(", {}", self.step_.unwrap_or(1))
            + &format!(", {}", self.ephemeral_)
            + &format!(", {:?}),\n", self.dsp_id);
        match self.array_ {
            None => format!("{}: {}", snake(&self.label), param),
            Some(length) => format!("{}: [\n{}],\n", snake(&self.label), param.repeat(length)),
        }
    }

    fn rust_visit_call(&self, label_suffix: &str, field_suffix: &str) -> String {
        format!("visitor.visit(\"{}{}\", ", self.label, label_suffix)
            + &format!("|state| &state.{}{});\n", snake(&self.label), field_suffix)
    }

    fn rust_visit(&self) -> String {
        match self.array_ {
            None => self.rust_visit_call("", ""),
            Some(length) => (0..length)
                .map(|i| self.rust_visit_call(&i.to_string(), &format!("[{}]", i)))
                .collect(),
        }
    }

    fn elm_field(&self, i: usize) -> String {
        let type_ = match self.type_ {
            Type::Bool(_) => "Bool",
            Type::I32(_) => "Int",
            Type::Usize(_) => "Int",
        };
        let type_ = match self.array_ {
            None => type_.to_owned(),
            Some(_) => format!("Array {}", type_),
        };
        format!(" {} {} : {}\n", if_(i == 0, "{", ","), self.label, type_)
    }

    fn elm_decoder(&self) -> String {
        let decoder = match self.type_ {
            Type::Bool(_) => "bool",
            Type::I32(_) => "int",
            Type::Usize(_) => "int",
        };
        let param = match self.array_ {
            None => String::from("Param.field"),
            Some(size) => format!("Param.array {} .{}", size, self.label),
        };
        String::new()
            + &format!(" |> Param.apply ({}", param)
            + &format!(" (\\x s -> {{ s | {} = x }})", self.label)
            + &format!(" Json.Decode.{} \"{}\")\n", decoder, self.label)
    }
}

fn generate_rust(params: &[Param], tag: &str) -> String {
    let mut buttons = String::new();
    let mut fields = String::new();
    let mut defaults = String::new();
    let mut visits = String::new();
    for param in params {
        if param.is_button() {
            buttons += &param.rust_button();
        } else {
            fields += &param.rust_field();
            defaults += &param.rust_default();
            visits += &param.rust_visit();
        }
    }
    let mut s = buttons;
    s += "#[allow(non_camel_case_types)]\n";
    s += "#[derive(Clone)]\n";
    s += &format!("pub struct {} {{\n{}}}\n\n", tag, fields);
    s += &format!("impl Default for {} {{\n", tag);
    s += &format!("fn default() -> Self {{\nSelf {{\n{}}}\n}}\n}}\n", defaults);
    s += &format!("impl IsState for {} {{\n", tag);
    s += "fn visit_params<T: Visitor<Self>>(visitor: &mut T) {\n";
    s += &format!("{}}}\n}}\n", visits);
    s
}

fn generate_elm(params: &[Param], tag: &str) -> String {
    let mut fields = String::new();
    let mut decoders = String::new();
    for (i, param) in params.iter().filter(|param| !param.is_button()).enumerate() {
        fields += &param.elm_field(i);
        decoders += &param.elm_decoder();
    }
    let mut s = String::new();
    s += &format!("module {} exposing (..)\n", tag);
    s += "import Param\n";
    s += "import Json.Decode\n";
    s += "import Array exposing (Array)\n";
    s += &format!("type alias {} =\n{} }}\n\n", tag, fields);
    s += &format!("decoder : Param.Decoder {} {}\n", tag, tag);
    s += &format!("decoder = Param.succeed {}\n{}", tag, decoders);
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
        Param::new("note", Type::I32(0)).group(15).ephemeral(),
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
            params.push(Param {
                label: faust_param.label,
                type_: Type::I32(faust_param.init.unwrap_or_default()),
                step_: faust_param.step,
                min_: faust_param.min.map(Type::I32),
                max_: faust_param.max.map(Type::I32),
                array_: None,
                ephemeral_: false,
                dsp_id: Some((stem.to_string_lossy().into_owned(), i)),
            });
        }
    }
    let _ = std::fs::create_dir_all("elm-stuff/typebeat");
    std::fs::write(&out.join("song.rs"), generate_rust(&song_params, "song"))?;
    std::fs::write(&out.join("track.rs"), generate_rust(&track_params, "track"))?;
    std::fs::write("elm-stuff/Song.elm", generate_elm(&song_params, "Song"))?;
    std::fs::write("elm-stuff/Track.elm", generate_elm(&track_params, "Track"))?;
    #[cfg(not(feature = "netlify"))]
    tauri_build::build();
    Ok(())
}
