extern crate anyhow;
extern crate atomic_float;
extern crate audrey;
extern crate cpal;
extern crate serde;

use anyhow::{Context, Result};
use atomic_float::AtomicF32;
use audrey::read::Reader;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleRate;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicU8, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcRequest, RpcResponse, WebViewBuilder};
use wry::Value;

const VOICE_COUNT: u8 = 5;
const TRACK_COUNT: u8 = 15;

struct SampleFile {
    channels: u32,
    samples: Vec<f32>,
}

impl SampleFile {
    fn read(i: u8) -> Result<Self> {
        let mut path = std::env::current_dir()?;
        path.push("audio");
        path.push("samples");
        path.push(format!("{:02}.wav", i));
        let mut reader = Reader::open(path)?;
        let channels = reader.description().channel_count();
        let sample_rate = reader.description().sample_rate();
        std::assert!(channels == 1 || channels == 2);
        std::assert_eq!(sample_rate, 44100);
        let samples = reader
            .samples()
            .collect::<Result<Vec<f32>, audrey::read::FormatError>>()?;
        Ok(Self { channels, samples })
    }

    fn sample(&self, position: usize, channel: usize) -> f32 {
        if self.mono() {
            self.samples[position]
        } else {
            self.samples[2 * position + channel]
        }
    }

    fn duration(&self) -> usize {
        self.samples.len() / self.channels as usize
    }

    fn mono(&self) -> bool {
        self.channels == 1
    }
}

struct Track {
    sample_file: SampleFile,
}

struct Voice {
    track_id: AtomicU8,
    position: AtomicF32,
    increment: AtomicF32,
}

struct State {
    active_track: AtomicI16,
    playing: AtomicBool,
    armed: AtomicBool,
    tempo: AtomicI16,
    root: AtomicI16,
    scale: AtomicI16,
    voices: Vec<Voice>,
    tracks: Vec<Track>,
}

impl State {
    fn new() -> Result<Self> {
        let mut state = State {
            active_track: AtomicI16::new(0),
            playing: AtomicBool::new(false),
            armed: AtomicBool::new(false),
            tempo: AtomicI16::new(120),
            root: AtomicI16::new(0),
            scale: AtomicI16::new(0),
            tracks: Vec::with_capacity(TRACK_COUNT.into()),
            voices: Vec::with_capacity(VOICE_COUNT.into()),
        };
        for i in 0..TRACK_COUNT {
            state.tracks.push(Track {
                sample_file: SampleFile::read(i)?,
            });
        }
        for _ in 0..VOICE_COUNT {
            state.voices.push(Voice {
                track_id: AtomicU8::new(0),
                position: AtomicF32::new(0.0),
                increment: AtomicF32::new(0.0),
            });
        }
        Ok(state)
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
    AuditionDown,
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

fn key_down(state: &State, track_id: u8) {
    let voice = &state.voices[0];
    voice.track_id.store(track_id, Ordering::Relaxed);
    voice.position.store(0.0, Ordering::Relaxed);
    voice.increment.store(1.0, Ordering::Relaxed);
}

fn interpolate(x: f32, a: f32, b: f32) -> f32 {
    a + x * (b - a)
}

fn on_audio(state: &State, receiver: &Receiver<(Setter, u8)>, audio: &mut [f32]) {
    let messages = receiver.try_iter();
    messages.for_each(|(setter, data)| match setter {
        Setter::ActiveTrack => set(&state.active_track, data),
        Setter::AuditionDown => key_down(&state, data),
        Setter::Play => toggle(&state.playing),
        Setter::Arm => toggle(&state.armed),
        Setter::Tempo => nudge(&state.tempo, data, 10),
        Setter::TempoTaps => set(&state.tempo, data),
        Setter::Root => nudge(&state.root, data, 7),
        Setter::Scale => set(&state.scale, data),
    });
    let mut iter = audio.iter_mut();
    while let Some((l, r)) = iter.next().zip(iter.next()) {
        *l = 0.0;
        *r = 0.0;
        for voice in state.voices.iter() {
            let track = &state.tracks[voice.track_id.load(Ordering::Relaxed) as usize];
            let position = voice.position.load(Ordering::Relaxed);
            let position_i = position.floor() as usize;
            let increment = voice.increment.load(Ordering::Relaxed);
            if position_i < track.sample_file.duration() && position_i as f32 == position {
                *l += track.sample_file.sample(position_i, 0);
                *r += track.sample_file.sample(position_i, 1);
                voice.position.fetch_add(increment, Ordering::Relaxed);
            } else if position_i + 1 < track.sample_file.duration() {
                *l += interpolate(
                    position - position_i as f32,
                    track.sample_file.sample(position_i, 0),
                    track.sample_file.sample(position_i + 1, 0),
                );
                *r += interpolate(
                    position - position_i as f32,
                    track.sample_file.sample(position_i, 1),
                    track.sample_file.sample(position_i + 1, 1),
                );
                voice.position.fetch_add(increment, Ordering::Relaxed);
            }
        }
    }
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

fn on_ui(state: &State, sender: &Sender<(Setter, u8)>, request: RpcRequest) -> Option<RpcResponse> {
    let params = request.params?;
    let mut message: Vec<Message> = serde_json::from_value(params).ok()?;
    match message.pop()? {
        Message::Get { method } => Some(RpcResponse::new_result(
            request.id,
            Some(get_state(&state, method)),
        )),
        Message::Set { method, data } => {
            let _ = sender.send((method, data));
            None
        }
    }
}

fn main() -> Result<()> {
    let device = cpal::default_host()
        .default_output_device()
        .context("no default audio device")?;
    let config = device.default_output_config()?;
    std::assert_eq!(config.channels(), 2);
    std::assert_eq!(config.sample_rate(), SampleRate(44100));

    let (sender, receiver) = std::sync::mpsc::channel();
    let audio_state = Arc::new(State::new()?);
    let ui_state = Arc::clone(&audio_state);
    let stream = device.build_output_stream(
        &config.into(),
        move |data, _| on_audio(&audio_state, &receiver, data),
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
        .with_rpc_handler(move |_, request| on_ui(&ui_state, &sender, request))
        .build()?;
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
