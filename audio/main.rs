mod parameter;
extern crate anyhow;
extern crate atomic_float;
extern crate miniaudio;
extern crate serde;

use std::convert::TryInto;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use miniaudio::{Decoder, DecoderConfig, Device, DeviceConfig, DeviceType, Format, FramesMut};
use parameter::Parameter;
use serde::Deserialize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcRequest, RpcResponse, WebViewBuilder};
use wry::Value;

const VOICE_COUNT: u8 = 5;
const TRACK_COUNT: u8 = 15;

struct SampleFile {
    samples: Vec<f32>,
}

impl SampleFile {
    fn read(i: u8) -> Result<Self> {
        let mut path = std::env::current_dir()?;
        path.push("audio");
        path.push("samples");
        path.push(format!("{:02}.wav", i));
        let config = DecoderConfig::new(Format::F32, 2, 44100);
        let mut decoder = Decoder::from_file(&path, Some(&config))?;
        let mut samples = vec![0.0; (2 * decoder.length_in_pcm_frames()).try_into()?];
        let mut frames = FramesMut::wrap(&mut samples[..], Format::F32, 2);
        decoder.read_pcm_frames(&mut frames);
        Ok(SampleFile { samples })
    }

    fn sample(&self, position: usize, channel: usize) -> f32 {
        self.samples[2 * position + channel]
    }

    fn duration(&self) -> usize {
        self.samples.len() / 2 as usize
    }
}

struct Track {
    sample_file: SampleFile,
}

struct Voice {
    active: Parameter,
    track_id: Parameter,
    position: Parameter,
    increment: Parameter,
}

struct State {
    active_track_id: Parameter,
    playing: Parameter,
    armed: Parameter,
    tempo: Parameter,
    root: Parameter,
    scale: Parameter,
    voices: Vec<Voice>,
    tracks: Vec<Track>,
}

impl State {
    fn new() -> Result<Self> {
        let mut state = State {
            active_track_id: Parameter::new(0.0).between(0, TRACK_COUNT - 1),
            playing: Parameter::binary(false),
            armed: Parameter::binary(false),
            tempo: Parameter::new(120.).between(0., 999.),
            root: Parameter::new(0.).between(-12., 12.),
            scale: Parameter::new(0.).between(0., 4.),
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
                active: Parameter::binary(false),
                track_id: Parameter::new(0.).between(0, TRACK_COUNT - 1),
                position: Parameter::new(0.),
                increment: Parameter::new(0.),
            });
        }
        Ok(state)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "context")]
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

fn key_down(state: &State, track_id: u8) {
    let voice = &state.voices[0];
    voice.active.set(1.);
    voice.track_id.set(track_id);
    voice.position.set(0.);
    voice.increment.set(1.);
}

fn play_sample<F: Fn(usize, &mut f32)>(voice: &Voice, frame: &mut [f32], write: F) {
    for (channel, sample) in frame.iter_mut().enumerate() {
        write(channel, sample);
    }
    voice
        .position
        .set(voice.position.get() + voice.increment.get());
}

fn on_audio(state: &State, receiver: &Receiver<(Setter, u8)>, output: &mut FramesMut) {
    let messages = receiver.try_iter();
    messages.for_each(|(setter, data)| match setter {
        Setter::ActiveTrack => state.active_track_id.set(data),
        Setter::AuditionDown => key_down(&state, data),
        Setter::Play => state.playing.toggle(),
        Setter::Arm => state.armed.toggle(),
        Setter::Tempo => state.tempo.nudge(data, 10.),
        Setter::TempoTaps => state.tempo.set(data),
        Setter::Root => state.root.nudge(data, 7.),
        Setter::Scale => state.scale.set(data),
    });
    for frame in output.frames_mut::<f32>() {
        for voice in state.voices.iter() {
            if voice.active.is_zero() {
                continue;
            }
            let track = &state.tracks[voice.track_id.get() as usize];
            let duration = track.sample_file.duration();
            let position = voice.position.get();
            let position_i = position.floor() as usize;
            if position_i < duration && position.fract() <= f32::EPSILON {
                play_sample(&voice, frame, |channel, sample| {
                    *sample += track.sample_file.sample(position_i, channel);
                });
            } else if position_i + 1 < duration {
                play_sample(&voice, frame, |channel, sample| {
                    let a = track.sample_file.sample(position_i, channel);
                    let b = track.sample_file.sample(position_i + 1, channel);
                    *sample += a + position.trunc() * (b - a);
                });
            }
        }
    }
}

fn get_state(state: &State, method: Getter) -> Value {
    match method {
        Getter::ActiveTrack => state.active_track_id.get().into(),
        Getter::Playing => state.playing.get().into(),
        Getter::Armed => state.armed.get().into(),
        Getter::Tempo => state.tempo.get().into(),
        Getter::Root => state.root.get().into(),
        Getter::Scale => state.scale.get().into(),
    }
}

fn on_ui(state: &State, sender: &Sender<(Setter, u8)>, request: RpcRequest) -> Option<RpcResponse> {
    let (message,) = serde_json::from_value(request.params?).ok()?;
    match message {
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
    let (sender, receiver) = std::sync::mpsc::channel();
    let audio_state = Arc::new(State::new()?);
    let ui_state = Arc::clone(&audio_state);
    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(44100);
    let mut device = Device::new(None, &device_config)?;
    device.set_data_callback(move |_, output, _input| on_audio(&audio_state, &receiver, output));
    device.start()?;

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
