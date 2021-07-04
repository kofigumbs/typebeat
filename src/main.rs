extern crate anyhow;
extern crate cpal;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleRate};
use std::default::Default;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcResponse, WebViewBuilder};
use wry::Value;

// ATOMIC PARAMETERS

trait Atomic<T> {
    fn relax(&self) -> T;
}

impl Atomic<bool> for AtomicBool {
    fn relax(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}

impl Atomic<u16> for AtomicU16 {
    fn relax(&self) -> u16 {
        self.load(Ordering::Relaxed)
    }
}

// STATE

struct State {
    activeTrack: AtomicU16,
    playing: AtomicBool,
    armed: AtomicBool,
    tempo: AtomicU16,
    root: AtomicU16,
    scale: AtomicU16,
}

impl State {
    fn receive(&self, method: &str) -> Option<Value> {
        match method {
            "receive:playing" => Some(self.playing.relax().into()),
            "receive:armed" => Some(self.armed.relax().into()),
            "receive:tempo" => Some(self.tempo.relax().into()),
            "receive:root" => Some(self.root.relax().into()),
            "receive:scale" => Some(self.scale.relax().into()),
            _ => None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            activeTrack: AtomicU16::new(0),
            playing: AtomicBool::new(false),
            armed: AtomicBool::new(false),
            tempo: AtomicU16::new(120),
            root: AtomicU16::new(120),
            scale: AtomicU16::new(120),
        }
    }
}

// MAIN LOOP

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
            let data = ui_state.receive(&request.method)?;
            Some(RpcResponse::new_result(request.id, Some(data)))
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
