extern crate anyhow;
extern crate cpal;
extern crate serde;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleRate};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, AtomicI16, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcResponse, WebViewBuilder};
use wry::Value;

struct State {
    active_track: AtomicI16,
    playing: AtomicBool,
    armed: AtomicBool,
    tempo: AtomicI16,
    root: AtomicI16,
    scale: AtomicI16,
}

impl State {
    fn new() -> Self {
        State {
            active_track: AtomicI16::new(0),
            playing: AtomicBool::new(false),
            armed: AtomicBool::new(false),
            tempo: AtomicI16::new(120),
            root: AtomicI16::new(0),
            scale: AtomicI16::new(0),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "context", rename_all = "lowercase")]
enum Message {
    Get { method: Getter },
    Set { method: Setter, data: u8 },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Getter {
    ActiveTrack,
    Playing,
    Armed,
    Tempo,
    Root,
    Scale,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Setter {
    ActiveTrack,
    Play,
    Arm,
    Tempo,
    TempoTaps,
    Root,
    Scale,
}

fn toggle(value: &AtomicBool) {
    value.fetch_xor(true, Ordering::Relaxed);
}

fn set(value: &AtomicI16, data: u8) {
    value.store(data.into(), Ordering::Relaxed);
}

fn nudge(value: &AtomicI16, data: u8, jump: i16) {
    let diff = match data {
        0 => -jump,
        1 => -1,
        2 => 1,
        3 => jump,
        _ => 0,
    };
    value.fetch_add(diff, Ordering::Relaxed);
}

fn get_state(state: &State, method: Getter) -> Value {
    match method {
        Getter::ActiveTrack => state.active_track.load(Ordering::Relaxed).into(),
        Getter::Playing => state.playing.load(Ordering::Relaxed).into(),
        Getter::Armed => state.armed.load(Ordering::Relaxed).into(),
        Getter::Tempo => state.tempo.load(Ordering::Relaxed).into(),
        Getter::Root => state.root.load(Ordering::Relaxed).into(),
        Getter::Scale => state.scale.load(Ordering::Relaxed).into(),
    }
}

fn set_state(state: &State, method: Setter, data: u8) {
    match method {
        Setter::ActiveTrack => set(&state.active_track, data),
        Setter::Play => toggle(&state.playing),
        Setter::Arm => toggle(&state.armed),
        Setter::Tempo => nudge(&state.tempo, data, 10),
        Setter::TempoTaps => set(&state.tempo, data),
        Setter::Root => nudge(&state.root, data, 7),
        Setter::Scale => set(&state.scale, data),
    }
}

fn on_audio(audio: &mut [f32], state: &State, receiver: &Receiver<(Setter, u8)>) {
    for (message, data) in receiver.try_iter() {
        set_state(state, message, data);
    }
    for sample in audio.iter_mut() {
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

    let audio_state = Arc::new(State::new());
    let ui_state = Arc::clone(&audio_state);
    let (sender, receiver) = channel();
    let stream = device.build_output_stream(
        &config.into(),
        move |data, _| on_audio(data, &audio_state, &receiver),
        |_| {}, // error callback
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
        .with_rpc_handler(move |_, request| {
            let params = request.params?;
            let mut message: Vec<Message> = serde_json::from_value(params).ok()?;
            match message.pop()? {
                Message::Get { method } => Some(RpcResponse::new_result(
                    request.id,
                    Some(get_state(&ui_state, method)),
                )),
                Message::Set { method, data } => {
                    let _ = sender.send((method, data));
                    None
                }
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
