use tauri::api::path::BaseDirectory;
use tauri::{Builder, Event, Manager, Menu, MenuItem, State, Submenu};

use typebeat::{Controller, Platform};

#[tauri::command]
fn get(method: &'_ str, state: State<'_, Controller>) -> Option<i32> {
    state.get(method)
}

#[tauri::command]
fn set(method: &'_ str, data: i32, state: State<'_, Controller>) {
    state.set(method, data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let root = tauri::api::path::resolve_path(
        context.config(),
        context.package_info(),
        "assets",
        Some(BaseDirectory::Resource),
    )?;
    let controller = typebeat::init(Platform { root })?;
    controller.start();
    let app = Builder::default()
        .menu(menu)
        .manage(controller)
        .invoke_handler(tauri::generate_handler![get, set])
        .build(context)?;
    app.run(|handle, event| match event {
        // FIXME(https://github.com/tauri-apps/tao/issues/208)
        #[cfg(target_os = "macos")]
        Event::Ready => {
            use cocoa::appkit::NSWindow;
            let handle = handle.clone();
            tauri::async_runtime::spawn(async move {
                let window =
                    handle.get_window("main").unwrap().ns_window().unwrap() as cocoa::base::id;
                unsafe { window.makeFirstResponder_(window.contentView()) };
            });
        }
        _ => {}
    });
    Ok(())
}
