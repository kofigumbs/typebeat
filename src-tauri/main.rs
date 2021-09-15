#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use std::path::Path;

use tauri::{Builder, State};
use typebeat::Controller;

#[tauri::command]
fn rpc(method: &'_ str, context: &'_ str, data: i32, state: State<'_, Controller>) -> Option<i32> {
    state.handle_rpc(context, method, data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = tauri::generate_context!();
    let resource_dir = if cfg!(debug_assertions) {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../static")
    } else {
        tauri::api::path::resource_dir(context.package_info()).unwrap()
    };
    Builder::default()
        .manage(typebeat::start(&resource_dir)?)
        .invoke_handler(tauri::generate_handler![rpc])
        .run(context)?;
    Ok(())
}
