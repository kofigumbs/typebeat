use std::error::Error;

use tauri::api::path::BaseDirectory;
use tauri::{Builder, Menu, MenuItem, State, Submenu};
use typebeat::Controller;

#[tauri::command]
fn rpc(method: &'_ str, context: &'_ str, data: i32, state: State<'_, Controller>) -> Option<i32> {
    state.handle_rpc(context, method, data)
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
    let samples = tauri::api::path::resolve_path(
        context.config(),
        context.package_info(),
        "../static/samples",
        Some(BaseDirectory::Resource),
    )?;
    Builder::default()
        .menu(menu)
        .manage(typebeat::start(&samples)?)
        .invoke_handler(tauri::generate_handler![rpc])
        .run(context)?;
    Ok(())
}
