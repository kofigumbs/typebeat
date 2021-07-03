extern crate anyhow;
extern crate cpal;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleRate};
use std::default::Default;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcResponse, WebViewBuilder};

struct State {
    tempo: AtomicU16,
}

impl Default for State {
    fn default() -> Self {
        State {
            tempo: AtomicU16::new(120),
        }
    }
}

fn data_callback(data: &mut [f32], state: &mut Arc<State>, receiver: &Receiver<String>) {
    for message in receiver.try_iter() {
        println!("{}", message);
    }
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}

fn main() -> Result<()> {
    let device = cpal::default_host()
        .default_output_device()
        .context("no default audio device")?;
    let config = device.default_output_config()?;
    std::assert_eq!(config.channels(), 2);
    std::assert_eq!(config.sample_rate(), SampleRate(44100));

    let ui_state: Arc<State> = Arc::new(Default::default());
    let mut audio_state = Arc::clone(&ui_state);
    let (sender, receiver) = channel();
    let stream = device.build_output_stream(
        &config.into(),
        move |data, _| data_callback(data, &mut audio_state, &receiver),
        |_| {},
    )?;
    stream.play()?;

    let mut path = std::env::current_dir()?;
    path.push("ui");
    path.push("index.html");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Typebeat")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", path.display()))?
        .with_rpc_handler(move |_window, request| {
            let _result = sender.send(request.method);
            Some(RpcResponse::new_result(
                request.id,
                Some(ui_state.tempo.load(Ordering::Relaxed).into()),
            ))
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
