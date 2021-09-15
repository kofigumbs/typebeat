use std::error::Error;

use tauri::api::path::BaseDirectory;
use tauri::{Builder, State};
use typebeat::Controller;

#[tauri::command]
fn rpc(method: &'_ str, context: &'_ str, data: i32, state: State<'_, Controller>) -> Option<i32> {
    state.handle_rpc(context, method, data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = tauri::generate_context!();
    let samples = tauri::api::path::resolve_path(
        context.config(),
        context.package_info(),
        "../static/samples",
        Some(BaseDirectory::Resource),
    )?;
    println!("{:?}", samples);
    Builder::default()
        .manage(typebeat::start(&samples)?)
        .invoke_handler(tauri::generate_handler![rpc])
        .run(context)?;
    Ok(())
}
