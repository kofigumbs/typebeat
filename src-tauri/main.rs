use std::error::Error;
use std::path::{Path, PathBuf};

use tauri::api::path::BaseDirectory;
use tauri::{Builder, Menu, MenuItem, State, Submenu};

use typebeat::{Controller, Platform};

struct TauriPlatform {
    root: PathBuf,
}

impl Platform for TauriPlatform {
    fn root(&self) -> &Path {
        &self.root
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
    let platform = TauriPlatform {
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
