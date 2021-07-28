use std::convert::TryInto;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;
use miniaudio::{
    Decoder, DecoderConfig, Device, DeviceConfig, DeviceType, Format, Frames, FramesMut,
};
use num_traits::Num;
use serde::Deserialize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcRequest, RpcResponse, WebViewBuilder};

const VOICE_COUNT: u8 = 5;
const TRACK_COUNT: u8 = 15;

struct SampleFile {
    samples: Vec<f32>,
}

impl SampleFile {
    fn read(i: u8) -> Result<Self> {
        let mut path = std::env::current_dir()?;
        path.push("src");
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

#[derive(Clone, Copy)]
enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}
impl From<u8> for SampleType {
    fn from(value: u8) -> Self {
        let values = [
            SampleType::File,
            SampleType::Live,
            SampleType::LiveRecord,
            SampleType::LivePlay,
        ];
        values[(value as usize).clamp(0, values.len() - 1)]
    }
}

struct Track {
    sample_file: SampleFile,
    sample_type: AtomicCell<SampleType>,
}

struct Voice {
    active: AtomicCell<bool>,
    track_id: AtomicCell<u8>,
    position: AtomicCell<f32>,
    increment: AtomicCell<f32>,
}

struct State {
    active_track_id: AtomicCell<u8>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: AtomicCell<u16>,
    root: AtomicCell<i8>,
    scale: AtomicCell<u8>,
    voices: Vec<Voice>,
    tracks: Vec<Track>,
}

impl State {
    fn new() -> Result<Self> {
        let mut state = State {
            active_track_id: 0.into(),
            playing: false.into(),
            armed: false.into(),
            tempo: 120.into(),
            root: 0.into(),
            scale: 0.into(),
            tracks: Vec::with_capacity(TRACK_COUNT.into()),
            voices: Vec::with_capacity(VOICE_COUNT.into()),
        };
        for i in 0..TRACK_COUNT {
            state.tracks.push(Track {
                sample_file: SampleFile::read(i)?,
                sample_type: AtomicCell::new(SampleType::File),
            });
        }
        for _ in 0..VOICE_COUNT {
            state.voices.push(Voice {
                active: false.into(),
                track_id: 0.into(),
                position: 0.0.into(),
                increment: 0.0.into(),
            });
        }
        Ok(state)
    }

    fn active_track(&self) -> &Track {
        &self.tracks[self.active_track_id.load() as usize]
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
    SampleType,
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
    SampleType,
}

enum Limit<T> {
    Below(T),
    Between(T, T),
}

fn set<T: Ord + Num>(atom: &AtomicCell<T>, value: T, limit: Limit<T>) {
    atom.store(match limit {
        Limit::Below(max) => value.clamp(T::zero(), max - T::one()),
        Limit::Between(min, max) => value.clamp(min, max),
    });
}

fn nudge<T: Copy + Ord + Num>(atom: &AtomicCell<T>, value: u8, jump: T, limit: Limit<T>) {
    match value {
        0 => set(atom, atom.load() - jump, limit),
        1 => set(atom, atom.load() - num_traits::one(), limit),
        2 => set(atom, atom.load() + num_traits::one(), limit),
        3 => set(atom, atom.load() + jump, limit),
        _ => {}
    }
}

fn toggle(atom: &AtomicCell<bool>) {
    atom.fetch_xor(true);
}

fn key_down(state: &State, track_id: u8) {
    let voice = &state.voices[0];
    voice.active.store(true);
    voice.track_id.store(track_id);
    voice.position.store(0.0);
    voice.increment.store(1.0);
}

fn play_sample<F: Fn(usize, &mut f32)>(voice: &Voice, frame: &mut [f32], write: F) {
    for (channel, sample) in frame.iter_mut().enumerate() {
        write(channel, sample);
    }
    voice
        .position
        .store(voice.position.load() + voice.increment.load());
}

fn on_audio(
    state: &State,
    receiver: &Receiver<(Setter, u8)>,
    input: &Frames,
    output: &mut FramesMut,
) {
    let messages = receiver.try_iter();
    messages.for_each(|(setter, data)| match setter {
        Setter::ActiveTrack => set(&state.active_track_id, data, Limit::Below(TRACK_COUNT)),
        Setter::AuditionDown => key_down(&state, data),
        Setter::Play => toggle(&state.playing),
        Setter::Arm => toggle(&state.armed),
        Setter::Tempo => nudge(&state.tempo, data, 10, Limit::Between(1, 999)),
        Setter::TempoTaps => set(&state.tempo, data as u16, Limit::Between(1, 999)),
        Setter::Root => nudge(&state.root, data, 7, Limit::Between(-12, 12)),
        Setter::Scale => set(&state.scale, data, Limit::Below(5)),
        Setter::SampleType => state.active_track().sample_type.store(data.into()),
    });
    for (frame_in, frame_out) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
        for voice in state.voices.iter() {
            if !voice.active.load() {
                continue;
            }
            let track = &state.tracks[voice.track_id.load() as usize];
            match track.sample_type.load() {
                SampleType::File => {
                    let duration = track.sample_file.duration();
                    let position = voice.position.load();
                    let position_i = position.floor() as usize;
                    if position_i < duration && position.fract() <= f32::EPSILON {
                        play_sample(&voice, frame_out, |channel, sample| {
                            *sample += track.sample_file.sample(position_i, channel);
                        });
                    } else if position_i + 1 < duration {
                        play_sample(&voice, frame_out, |channel, sample| {
                            let a = track.sample_file.sample(position_i, channel);
                            let b = track.sample_file.sample(position_i + 1, channel);
                            *sample += a + position.trunc() * (b - a);
                        });
                    }
                }
                SampleType::Live | SampleType::LiveRecord | SampleType::LivePlay => {
                    for sample in frame_out.iter_mut() {
                        *sample += frame_in.iter().sum::<f32>();
                    }
                }
            }
        }
    }
}

fn get_state(state: &State, method: Getter) -> i32 {
    match method {
        Getter::ActiveTrack => state.active_track_id.load().into(),
        Getter::Playing => state.playing.load().into(),
        Getter::Armed => state.armed.load().into(),
        Getter::Tempo => state.tempo.load().into(),
        Getter::Root => state.root.load().into(),
        Getter::Scale => state.scale.load().into(),
        Getter::SampleType => state.active_track().sample_type.load() as i32,
    }
}

fn on_ui(state: &State, sender: &Sender<(Setter, u8)>, request: RpcRequest) -> Option<RpcResponse> {
    let (message,) = serde_json::from_value(request.params?).ok()?;
    match message {
        Message::Get { method } => Some(RpcResponse::new_result(
            request.id,
            Some(get_state(&state, method).into()),
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
    device.set_data_callback(move |_, output, input| {
        on_audio(&audio_state, &receiver, input, output)
    });
    device.start()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Typebeat")
        .build(&event_loop)?;
    let ui = std::env::current_dir()?
        .join("src")
        .join("ui")
        .join("index.html");
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", ui.display()))?
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
