use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use serde::Serialize;
use serde_json::Value;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{
    AppHandle, Builder, CustomMenuItem, Event, Manager, Menu, MenuItem, State, Submenu, Window, Wry,
};

use typebeat::{Change, Controller, Platform, Strategy};

#[tauri::command]
fn dump(state: State<Controller>) -> impl Serialize {
    state.save(Strategy::Dump)
}

#[tauri::command]
fn send(method: &'_ str, data: i32, state: State<Controller>) {
    state.send(method, data)
}

fn dialog(window: &Window) -> FileDialogBuilder {
    let mut builder = FileDialogBuilder::new()
        .set_parent(window)
        .add_filter("Typebeat Save", &["typebeat"]);
    if let Some(audio) = tauri::api::path::audio_dir() {
        builder = builder.set_directory(audio);
    }
    builder
}

fn menu() -> Menu {
    Menu::new()
        .add_submenu(Submenu::new(
            "Typebeat",
            Menu::new()
                .add_native_item(MenuItem::CloseWindow)
                .add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "File",
            Menu::new()
                .add_item(CustomMenuItem::new("open", "Open").accelerator("CmdOrControl+O"))
                .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrControl+S")),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste),
        ))
}

fn open(window: &Window, handle: &AppHandle<Wry>) {
    let handle = handle.clone();
    dialog(&window).pick_file(move |path| {
        path.map(move |path| {
            let state: State<Controller> = handle.state();
            std::fs::read(path)
                .ok()
                .and_then(|file| serde_json::from_slice(&file).ok())
                .map(|save| state.load(&save));
        });
    });
}

fn save(window: &Window, handle: &AppHandle<Wry>) {
    let handle = handle.clone();
    dialog(&window).save_file(move |path| {
        path.map(move |path| {
            let state: State<Controller> = handle.state();
            let save = state.save(Strategy::File);
            let json = serde_json::to_vec(&save).expect("json");
            std::fs::write(path, json).expect("write");
        });
    });
}

fn on_ready(receiver: &Arc<Mutex<Receiver<Change>>>, handle: &AppHandle<Wry>) {
    let window = handle.get_window("main").expect("window");

    {
        let handle = handle.clone();
        let window_ = window.clone();
        window.on_menu_event(move |event| match event.menu_item_id() {
            "open" => open(&window_, &handle),
            "save" => save(&window_, &handle),
            _ => {}
        });
    }

    // FIXME(https://github.com/tauri-apps/tao/issues/208)
    #[cfg(target_os = "macos")]
    {
        let window = window.clone();
        tauri::async_runtime::spawn(async move {
            use cocoa::appkit::NSWindow;
            let window = window.ns_window().unwrap() as cocoa::base::id;
            unsafe { window.makeFirstResponder_(window.contentView()) };
        });
    }

    // Inform UI of param changes
    let receiver = Arc::clone(&receiver);
    std::thread::spawn(move || {
        let receiver = receiver.lock().expect("receiver");
        while let Ok(change) = receiver.recv() {
            window.emit("change", Some(change)).expect("emit");
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let context = tauri::generate_context!();
    let platform = Platform {
        voice_count: 8,
        root: tauri::api::path::resource_dir(context.package_info()).expect("root"),
        sender,
    };
    let controller = typebeat::init(platform, &Value::Null)?;
    controller.start();

    let app = Builder::default()
        .menu(menu())
        .manage(controller)
        .invoke_handler(tauri::generate_handler![dump, send])
        .build(context)?;
    let receiver = Arc::new(Mutex::new(receiver));
    app.run(move |handle, event| match event {
        Event::Ready => on_ready(&receiver, handle),
        _ => {}
    });

    Ok(())
}
