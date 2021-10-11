use std::sync::{Arc, Mutex};

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
    // Start Typebeat
    let (sender, receiver) = std::sync::mpsc::channel();
    let context = tauri::generate_context!();
    let root = tauri::api::path::resolve_path(
        context.config(),
        context.package_info(),
        "assets",
        Some(BaseDirectory::Resource),
    )?;
    let controller = typebeat::init(Platform { root, sender })?;
    controller.start();

    // Build Tauri app
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "Typebeat",
            Menu::new()
                .add_native_item(MenuItem::CloseWindow)
                .add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste),
        ));
    let app = Builder::default()
        .menu(menu)
        .manage(controller)
        .invoke_handler(tauri::generate_handler![get, set])
        .build(context)?;
    let receiver = Arc::new(Mutex::new(receiver));
    app.run(move |handle, event| match event {
        Event::Ready => {
            let window = handle.get_window("main").expect("window");

            // FIXME(https://github.com/tauri-apps/tao/issues/208)
            #[cfg(target_os = "macos")]
            {
                let window_ = window.clone();
                tauri::async_runtime::spawn(async move {
                    use cocoa::appkit::NSWindow;
                    let window_ = window_.ns_window().unwrap() as cocoa::base::id;
                    unsafe { window_.makeFirstResponder_(window_.contentView()) };
                });
            }

            // Inform UI of dirty state keys
            let receiver = Arc::clone(&receiver);
            std::thread::spawn(move || {
                let receiver = receiver.lock().expect("receiver");
                while let Ok(changed) = receiver.recv() {
                    window.emit("dirty", Some(changed)).expect("emit");
                }
            });
        }
        _ => {}
    });

    Ok(())
}
