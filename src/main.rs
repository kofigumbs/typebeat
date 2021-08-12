#![feature(array_methods)]

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};
use serde::Deserialize;
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{RpcRequest, RpcResponse, WebViewBuilder};

use bounded::Bounded;
use effects::{Bus, FaustDsp, ParamIndex, UI};
use samples::Sample;

mod bounded;
mod effects;
mod samples;

const VOICE_COUNT: i32 = 5;
const TRACK_COUNT: i32 = 15;
const SAMPLE_RATE: i32 = 44100;

fn add_assign_each(destination: &mut [f32], source: &[f32]) {
    for (destination, source) in destination.iter_mut().zip(source) {
        *destination += source;
    }
}

fn new_dsp<T: FaustDsp>() -> T {
    let mut dsp = T::new();
    dsp.instance_init(SAMPLE_RATE);
    dsp
}

#[derive(Clone, Copy)]
enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}
impl From<i32> for SampleType {
    fn from(value: i32) -> Self {
        match value {
            i if i <= 0 => SampleType::File,
            i if i == 1 => SampleType::Live,
            i if i == 2 => SampleType::LiveRecord,
            _ /* i > */ => SampleType::LivePlay,
        }
    }
}

#[derive(Default)]
struct EffectUi<T> {
    buttons: HashMap<&'static str, (ParamIndex, AtomicCell<bool>)>,
    entries: HashMap<&'static str, (ParamIndex, T, bounded::Dynamic<T>)>,
}
impl<T> UI<T> for EffectUi<T> {
    fn add_button(&mut self, s: &'static str, i: ParamIndex) {
        self.buttons.insert(s, (i, false.into()));
    }
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, value: T, min: T, max: T, step: T) {
        let entry = bounded::Dynamic {
            atom: value.into(),
            min,
            max,
        };
        self.entries.insert(s, (i, step, entry));
    }
}
impl EffectUi<f32> {
    fn set_params_on(&self, dsp: &mut dyn effects::FaustDsp<T = f32>) {
        for (index, _, value) in self.entries.values() {
            dsp.set_param(*index, value.load());
        }
    }
}

struct Track {
    file_sample: Sample,
    sample_type: AtomicCell<SampleType>,
    effect_ui: EffectUi<f32>,
}

#[derive(Default)]
struct Controls {
    active_track_id: bounded::Int<0, { TRACK_COUNT - 1 }>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: bounded::Int<1, 999>,
    root: bounded::Int<-12, 12>,
    scale: bounded::Int<0, 4>,
    tracks: Vec<Track>,
    sends: Vec<EffectUi<f32>>,
}
impl Controls {
    fn active_track(&self) -> &Track {
        &self.tracks[self.active_track_id.load() as usize]
    }
    fn effect_uis(&self) -> impl Iterator<Item = &EffectUi<f32>> {
        std::array::from_ref(&self.active_track().effect_ui)
            .iter()
            .chain(self.sends.iter())
    }
    fn find<T>(&self, f: impl Fn(&EffectUi<f32>) -> Option<&T>) -> Option<&T> {
        self.effect_uis().find_map(f)
    }
}

#[derive(Default)]
struct Cursor {
    age: usize,
    position: f32,
    increment: f32,
    track_id: Option<usize>,
}
impl Cursor {
    fn process(&mut self, track: &Track, input: &[f32], output: &mut [f32]) {
        match track.sample_type.load() {
            SampleType::File => {
                let duration = track.file_sample.duration();
                let position = self.position.floor() as usize;
                let position_rem = self.position.fract();
                if position < duration && position_rem == 0. {
                    self.advance(output, |channel, sample| {
                        *sample += track.file_sample.at(position, channel);
                    });
                } else if position + 1 < duration {
                    self.advance(output, |channel, sample| {
                        let a = track.file_sample.at(position, channel);
                        let b = track.file_sample.at(position + 1, channel);
                        *sample += a + position_rem * (b - a);
                    });
                }
            }
            SampleType::Live | SampleType::LiveRecord | SampleType::LivePlay => {
                for sample in output.iter_mut() {
                    *sample += input.iter().sum::<f32>();
                }
            }
        }
    }
    fn advance<F: Fn(usize, &mut f32)>(&mut self, frame: &mut [f32], write: F) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            write(channel, sample);
        }
        self.position += self.increment;
    }
}

struct Player {
    cursor: Cursor,
    insert: effects::insert,
}
impl Player {
    fn process(&mut self, track: &Track, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::INPUTS];
        let mut post_buffer = [0.; effects::insert::OUTPUTS];
        self.cursor.process(track, input, &mut pre_buffer);
        self.insert.compute(
            1,
            &pre_buffer.each_ref().map(std::slice::from_ref),
            &mut post_buffer.each_mut().map(std::slice::from_mut),
        );
        add_assign_each(output, &post_buffer);
    }
}

struct Voices {
    players: Vec<Player>,
    sends: Vec<Box<dyn Send + FaustDsp<T = f32>>>,
}
impl Voices {
    fn key_down(&mut self, track_id: i32) {
        for player in self.players.iter_mut() {
            player.cursor.age += 1;
        }
        let mut player = self
            .players
            .iter_mut()
            .max_by_key(|player| player.cursor.age)
            .unwrap();
        player.cursor = Cursor {
            age: 0,
            position: 0.0,
            increment: 1.0,
            track_id: Some(track_id as usize),
        };
        player.insert.instance_clear();
    }
    fn process(&mut self, controls: &Controls, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::OUTPUTS];
        let mut post_buffer = [0.; 2];
        for player in self.players.iter_mut() {
            let track = match player.cursor.track_id {
                None => continue,
                Some(track_id) => &controls.tracks[track_id],
            };
            track.effect_ui.set_params_on(&mut player.insert);
            player.process(track, input, &mut pre_buffer);
            add_assign_each(output, &pre_buffer);
        }
        for (i, (dsp, ui)) in self.sends.iter_mut().zip(controls.sends.iter()).enumerate() {
            ui.set_params_on(dsp.as_mut());
            dsp.compute(
                1,
                &pre_buffer.each_ref().map(std::slice::from_ref)[2 * (i + 1)..],
                &mut post_buffer.each_mut().map(std::slice::from_mut),
            );
            add_assign_each(output, &post_buffer);
        }
    }
}

enum Setter {
    Toggle(fn(&mut Audio) -> &AtomicCell<bool>),
    Number(fn(&mut Audio, i32)),
    Entry(&'static str),
}

struct Audio {
    controls: Arc<Controls>,
    voices: Voices,
    receiver: Receiver<(Setter, i32)>,
}
impl Audio {
    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        while let Ok((setter, data)) = self.receiver.try_recv() {
            match setter {
                Setter::Toggle(f) => {
                    f(self).fetch_xor(true);
                }
                Setter::Number(f) => {
                    f(self, data);
                }
                Setter::Entry(key) => {
                    if let Some((_, step, value)) = self.controls.find(|ui| ui.entries.get(&key)) {
                        match *step as i32 {
                            0 => value.store(if value.load() == 0. { 1. } else { 0. }),
                            1 => value.store(data as f32),
                            _ => value.nudge(data, *step),
                        }
                    }
                }
            }
        }
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            self.voices.process(&self.controls, input, output);
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "context")]
enum UiMessage {
    Get { method: String },
    Set { method: String, data: i32 },
}

struct Ui {
    controls: Arc<Controls>,
    sender: Sender<(Setter, i32)>,
}
impl Ui {
    fn process(&self, request: RpcRequest) -> Option<RpcResponse> {
        let (message,) = serde_json::from_value(request.params?).ok()?;
        match message {
            UiMessage::Get { method } => Some(RpcResponse::new_result(
                request.id,
                Some(self.get(&method).into()),
            )),
            UiMessage::Set { method, data } => {
                let _ = self.sender.send((self.set(&method), data));
                None
            }
        }
    }
    fn get(&self, method: &str) -> i32 {
        match method {
            "activeTrack" => self.controls.active_track_id.load().into(),
            "playing" => self.controls.playing.load().into(),
            "armed" => self.controls.armed.load().into(),
            "tempo" => self.controls.tempo.load().into(),
            "root" => self.controls.root.load().into(),
            "scale" => self.controls.scale.load().into(),
            "sampleType" => self.controls.active_track().sample_type.load() as i32,
            _ => self
                .controls
                .find(|ui| ui.entries.get(method))
                .map_or(0, |(_, _, value)| value.load() as i32),
        }
    }
    fn set(&self, method: &str) -> Setter {
        match method {
            "play" => Setter::Toggle(|audio| &audio.controls.playing),
            "arm" => Setter::Toggle(|audio| &audio.controls.armed),
            "activeTrack" => Setter::Number(|audio, i| audio.controls.active_track_id.store(i)),
            "auditionDown" => Setter::Number(|audio, i| audio.voices.key_down(i)),
            "tempo" => Setter::Number(|audio, i| audio.controls.tempo.nudge(i, 10)),
            "tempoTaps" => Setter::Number(|audio, i| audio.controls.tempo.store(i)),
            "root" => Setter::Number(|audio, i| audio.controls.root.nudge(i, 7)),
            "scale" => Setter::Number(|audio, i| audio.controls.scale.store(i)),
            "sampleType" => Setter::Number(|audio, i| {
                audio.controls.active_track().sample_type.store(i.into());
            }),
            _ => Setter::Entry(
                self.controls
                    .find(|ui| ui.entries.get_key_value(method).map(|(key, _)| key))
                    .map_or(&"", |key| *key),
            ),
        }
    }
}

fn main() -> Result<()> {
    let mut controls = Controls::default();
    controls.tempo.store(120);
    for i in 0..TRACK_COUNT {
        let mut effect_ui = EffectUi::default();
        effects::insert::build_user_interface_static(&mut effect_ui);
        controls.tracks.push(Track {
            file_sample: Sample::read_file(i)?,
            sample_type: SampleType::File.into(),
            effect_ui,
        });
    }

    let mut players = Vec::new();
    players.resize_with(VOICE_COUNT as usize, || Player {
        cursor: Cursor::default(),
        insert: new_dsp::<effects::insert>(),
    });

    let voices = Voices {
        players,
        sends: vec![
            Box::new(new_dsp::<effects::reverb>()),
            Box::new(new_dsp::<effects::echo>()),
            Box::new(new_dsp::<effects::drive>()),
        ],
    };
    for dsp in voices.sends.iter() {
        let mut ui = EffectUi::default();
        dsp.build_user_interface(&mut ui);
        controls.sends.push(ui);
    }
    for player in voices.players.iter() {
        assert_eq!(player.insert.get_num_inputs(), 2);
        assert_eq!(
            voices.sends.len() + 1,
            player.insert.get_num_outputs() as usize / 2
        );
    }
    for dsp in voices.sends.iter() {
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let ui = Ui {
        sender,
        controls: Arc::new(controls),
    };
    let mut audio = Audio {
        receiver,
        controls: Arc::clone(&ui.controls),
        voices,
    };

    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(SAMPLE_RATE as u32);
    let mut device = Device::new(None, &device_config)?;
    device.set_data_callback(move |_, output, input| audio.process(input, output));
    device.start()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Typebeat")
        .build(&event_loop)?;
    let html = std::env::current_dir()?
        .join("src")
        .join("ui")
        .join("index.html");
    let _webview = WebViewBuilder::new(window)?
        .with_url(&format!("file://{}", html.display()))?
        .with_rpc_handler(move |_, request| ui.process(request))
        .build()?;
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
