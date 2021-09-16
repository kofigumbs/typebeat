use std::error::Error;
use std::path::PathBuf;

use tauri::api::path::BaseDirectory;
use tauri::{Builder, Menu, MenuItem, State, Submenu};

use typebeat::miniaudio::{Decoder, DecoderConfig, Format, FramesMut};
use typebeat::{Controller, Platform, SAMPLE_RATE};

struct FilePlatform {
    root: PathBuf,
}

impl Platform for FilePlatform {
    fn get_stereo_sample(&self, i: usize) -> Vec<f32> {
        let path = self.root.join("samples").join(format!("{:02}.wav", i));
        let config = DecoderConfig::new(Format::F32, 2, SAMPLE_RATE as u32);
        let mut decoder = Decoder::from_file(&path, Some(&config)).expect("decoder");
        let mut samples = vec![0.0; 2 * decoder.length_in_pcm_frames() as usize];
        decoder.read_pcm_frames(&mut FramesMut::wrap(&mut samples[..], Format::F32, 2));
        samples
    }
}

#[tauri::command]
fn get(method: &'_ str, state: State<'_, Controller>) -> Option<i32> {
    state.get(method)
}

#[tauri::command]
fn set(method: &'_ str, data: i32, state: State<'_, Controller>) {
    state.set(method, data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "Typebeat",
            Menu::new().add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste),
        ));
    let context = tauri::generate_context!();
    let platform = FilePlatform {
        root: tauri::api::path::resolve_path(
            context.config(),
            context.package_info(),
            "../static",
            Some(BaseDirectory::Resource),
        )?,
    };
    Builder::default()
        .menu(menu)
        .manage(typebeat::start(platform)?)
        .invoke_handler(tauri::generate_handler![get, set])
        .run(context)?;
    Ok(())
}
