use anyhow::Result;
use wry::application::dpi::LogicalSize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcResponse, WebViewBuilder};

pub fn start(f: impl Fn(&str, &str, i32) -> Option<i32> + 'static) -> Result<!> {
    let event_loop = EventLoop::new();
    let size = LogicalSize::new(1200., 415.);
    let window_builder = WindowBuilder::new()
        .with_title("Typebeat")
        .with_inner_size(size)
        .with_min_inner_size(size);
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
                let response = f(context, method, data);
                Some(RpcResponse::new_result(request.id, response.map(i32::into)))
            }
        })
        .build()?;
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
