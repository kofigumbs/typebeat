use std::sync::Arc;

use anyhow::{Context, Result};
use wry::application::accelerator::{Accelerator, SysMods};
use wry::application::dpi::LogicalSize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::keyboard::KeyCode;
use wry::application::menu::{MenuBar, MenuItem, MenuItemAttributes};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcResponse, WebViewBuilder};

pub trait Handler {
    fn on_open(&self);
    fn on_save(&self);
    fn on_rpc(&self, context: &str, method: &str, data: i32) -> Option<i32>;
}

pub fn start<T: Handler + 'static>(handler: T) -> Result<!> {
    let rpc_handler = Arc::new(handler);
    let event_loop_handler = Arc::clone(&rpc_handler);

    let mut menu = MenuBar::new();
    let mut main_submenu = MenuBar::new();
    main_submenu.add_native_item(MenuItem::Quit).context("⌘Q")?;
    menu.add_submenu("Typebeat", true, main_submenu);

    let mut file_submenu = MenuBar::new();
    let open = file_submenu.add_item(
        MenuItemAttributes::new("Open")
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyO)),
    );
    let save = file_submenu.add_item(
        MenuItemAttributes::new("Save")
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyS)),
    );
    menu.add_submenu("File", true, file_submenu);

    let event_loop = EventLoop::new();
    let size = LogicalSize::new(1200., 415.);
    let window_builder = WindowBuilder::new()
        .with_title("Typebeat")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_menu(menu);
    let window;
    #[cfg(target_os = "macos")]
    {
        use wry::application::platform::macos::WindowBuilderExtMacOS;
        window = window_builder
            .with_title_hidden(true)
            .with_fullsize_content_view(true)
            .with_titlebar_buttons_hidden(true)
            .with_titlebar_transparent(true)
            .build(&event_loop)?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        window = window_builder.with_decorations(false).build(&event_loop)?;
    }
    let html = std::env::current_dir()?
        .join("src")
        .join("ui")
        .join("index.html");
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", html.display()))?
        .with_rpc_handler(move |window, request| match request.method.as_str() {
            "maximize" => {
                window.set_maximized(!window.is_maximized());
                None
            }
            "move" => {
                let _ = window.drag_window();
                None
            }
            _ => {
                let param = &request.params?[0];
                let context = param["context"].as_str()?;
                let method = param["method"].as_str()?;
                let data = param["data"].as_i64()? as i32;
                let response = rpc_handler.on_rpc(context, method, data);
                Some(RpcResponse::new_result(request.id, response.map(i32::into)))
            }
        })
        .build()?;
    event_loop.run(move |event, _, control_flow| match event {
        Event::MenuEvent { menu_id, .. } if menu_id == open.clone().id() => {
            event_loop_handler.on_open();
            *control_flow = ControlFlow::Wait
        }
        Event::MenuEvent { menu_id, .. } if menu_id == save.clone().id() => {
            event_loop_handler.on_save();
            *control_flow = ControlFlow::Wait
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
