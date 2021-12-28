#![feature(format_args_capture)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use serde::Serialize;
use serde_json::Value;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{
    AppHandle, Builder, CustomMenuItem, Event, Manager, Menu, MenuItem, State, Submenu, Window, Wry,
};

use typebeat::{AtomicCell, Change, Controller, Platform};

struct App {
    controller: Controller,
    labels: AtomicCell<Option<bool>>,
    location: Mutex<(Option<PathBuf>, Option<String>)>,
}

impl From<Controller> for App {
    fn from(controller: Controller) -> App {
        App {
            controller,
            labels: None.into(),
            location: (tauri::api::path::audio_dir(), None).into(),
        }
    }
}

#[tauri::command]
fn dump(state: State<App>) -> impl Serialize {
    state.controller.dump()
}

#[tauri::command]
fn send(method: &'_ str, data: i32, state: State<App>) {
    state.controller.send(method, data)
}

#[tauri::command]
fn labels(setting: bool, window: Window, state: State<App>) {
    let title = format!("{} Keyboard Labels", if setting { "Hide" } else { "Show" });
    window
        .menu_handle()
        .get_item("labels")
        .set_title(&title)
        .expect("set_title");
    state.labels.store(Some(setting));
}

#[tauri::command]
fn theme(setting: &'_ str, window: Window) {
    for &theme in themes().iter() {
        window
            .menu_handle()
            .get_item(theme)
            .set_selected(theme == setting)
            .expect("set_selected");
    }
}

fn menu() -> Menu {
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "Typebeat",
            Menu::new()
                .add_native_item(MenuItem::CloseWindow)
                .add_native_item(MenuItem::Quit),
        ));
    }
    menu.add_submenu(Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("new", "New").accelerator("CmdOrControl+Shift+N"))
            .add_item(CustomMenuItem::new("open", "Open").accelerator("CmdOrControl+O"))
            .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrControl+S")),
    ))
    .add_submenu(Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    ))
    .add_submenu(Submenu::new(
        "View",
        Menu::new()
            .add_submenu(Submenu::new(
                "Themes",
                themes().iter().fold(Menu::new(), |menu, &theme| {
                    menu.add_item(CustomMenuItem::new(theme, theme))
                }),
            ))
            .add_item(CustomMenuItem::new("labels", "Keyboard Labels")),
    ))
    .add_submenu(Submenu::new(
        "Help",
        Menu::new().add_item(CustomMenuItem::new("demo", "Typebeat Demo")),
    ))
}

fn themes() -> &'static [&'static str] {
    &["Gruvbox", "Pencil", "Solarized", "Mute"]
}

fn dialog(window: &Window) -> FileDialogBuilder {
    let mut builder = FileDialogBuilder::new()
        .set_parent(window)
        .add_filter("Typebeat Save", &["typebeat"]);
    if let Ok(location) = window.state::<App>().location.lock() {
        if let Some(directory) = &location.0 {
            builder = builder.set_directory(directory);
        }
        if let Some(file_name) = &location.1 {
            builder = builder.set_file_name(file_name);
        }
    }
    builder
}

fn with_location(
    window: &Window,
    handle: &AppHandle<Wry>,
    f: impl FnOnce(&App, PathBuf) + Send + 'static,
) -> impl FnOnce(Option<PathBuf>) + Send + 'static {
    let window = window.clone();
    let handle = handle.clone();
    move |path| {
        if let Some(path) = path {
            let state: State<App> = handle.state();
            let parent = path.parent().map(Path::to_path_buf);
            let file_name = path.file_name().map(|x| x.to_string_lossy().into());
            *state.location.lock().expect("location") = (parent, file_name.clone());
            file_name.map(|file_name| window.set_title(&file_name));
            f(&state, path);
        }
    }
}

fn open(window: &Window, handle: &AppHandle<Wry>) {
    dialog(window).pick_file(with_location(window, handle, |state, path| {
        std::fs::read(path)
            .ok()
            .and_then(|file| serde_json::from_slice(&file).ok())
            .map(|json| state.controller.load(&json));
    }));
}

fn save(window: &Window, handle: &AppHandle<Wry>) {
    dialog(window).save_file(with_location(window, handle, |state, path| {
        let json = serde_json::to_vec(&state.controller.save()).expect("json");
        std::fs::write(path, json).expect("write");
    }));
}

fn on_ready(receiver: &Arc<Mutex<Receiver<Change>>>, handle: &AppHandle<Wry>) {
    let window = handle.get_window("main").expect("window");

    // Setup menu handlers
    let window_ = window.clone();
    let handle_ = handle.clone();
    window.on_menu_event(move |event| {
        let state = handle_.state::<App>();
        match event.menu_item_id() {
            "new" => state.controller.load(&Value::Null),
            "open" => open(&window_, &handle_),
            "save" => save(&window_, &handle_),
            "demo" => tauri::api::shell::open("https://typebeat.xyz".into(), None).expect("demo"),
            "labels" => window_
                .emit("labels", Some(!state.labels.load().expect("labels")))
                .expect("emit"),
            id if themes().contains(&id) => window_.emit("theme", Some(id)).expect("emit"),
            _ => {}
        }
    });

    // Setup state change events in JavaScript
    let window_ = window.clone();
    let receiver = Arc::clone(&receiver);
    std::thread::spawn(move || {
        let receiver = receiver.lock().expect("receiver");
        while let Ok(change) = receiver.recv() {
            window_.emit("change", Some(change)).expect("emit");
        }
    });

    // FIXME(https://github.com/tauri-apps/tao/issues/208)
    #[cfg(target_os = "macos")]
    {
        tauri::async_runtime::spawn(async move {
            use cocoa::appkit::NSWindow;
            let window = window.ns_window().unwrap() as cocoa::base::id;
            unsafe { window.makeFirstResponder_(window.contentView()) };
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let context = tauri::generate_context!();
    let platform = Platform {
        voice_count: 12,
        root: tauri::api::path::resource_dir(context.package_info()).expect("root"),
        sender,
    };
    let controller = typebeat::init(platform, &Value::Null)?;
    controller.start();

    let app = Builder::default()
        .menu(menu())
        .manage(App::from(controller))
        .invoke_handler(tauri::generate_handler![dump, send, labels, theme])
        .build(context)?;
    let receiver = Arc::new(Mutex::new(receiver));
    app.run(move |handle, event| match event {
        Event::Ready => on_ready(&receiver, handle),
        _ => {}
    });

    Ok(())
}
