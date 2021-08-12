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

const SCALE_OFFSETS: [[i32; 7]; 4] = [
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
    [0, 2, 3, 5, 7, 9, 11],
];

fn bool_to_float(x: bool) -> f32 {
    return if x { 1. } else { 0. };
}

fn add_assign_each(destination: &mut [f32], source: &[f32]) {
    for (destination, source) in destination.iter_mut().zip(source) {
        *destination += source;
    }
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
    map: HashMap<&'static str, (ParamIndex, T, bounded::Dynamic<T>)>,
}
impl<T: From<f32>> UI<T> for EffectUi<T> {
    fn add_button(&mut self, s: &'static str, i: ParamIndex) {
        self.add_num_entry(s, i, 0.0.into(), 0.0.into(), 1.0.into(), 0.0.into());
    }
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, value: T, min: T, max: T, step: T) {
        let entry = bounded::Dynamic {
            atom: value.into(),
            min,
            max,
        };
        self.map.insert(s, (i, step, entry));
    }
}
impl EffectUi<f32> {
    fn store(&self, label: &'static str, data: f32) {
        if let Some((_, _, value)) = &self.map.get(label) {
            value.store(data);
        }
    }
    fn set_params_on(&self, dsp: &mut dyn effects::FaustDsp<T = f32>) {
        for (index, _, value) in self.map.values() {
            dsp.set_param(*index, value.load());
        }
    }
}

struct Track {
    file_sample: Sample,
    sample_type: AtomicCell<SampleType>,
    octave: bounded::Int<2, 8>,
    use_key: AtomicCell<bool>,
    insert: EffectUi<f32>,
}
impl Track {
    fn note(&self, root: i32, scale: i32, key: i32) -> i32 {
        (self.octave.load() + key / 7) * 12
            + if self.use_key.load() {
                SCALE_OFFSETS[scale as usize][key as usize % 7] + root
            } else {
                SCALE_OFFSETS[0][key as usize % 7]
            }
    }
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
        std::array::from_ref(&self.active_track().insert)
            .iter()
            .chain(self.sends.iter())
    }
    fn find<T>(&self, f: impl Fn(&EffectUi<f32>) -> Option<&T>) -> Option<&T> {
        self.effect_uis().find_map(f)
    }
}

struct Dsp<T> {
    dsp: T,
}
impl<T: FaustDsp> Default for Dsp<T> {
    fn default() -> Self {
        let mut dsp = T::new();
        dsp.instance_init(SAMPLE_RATE);
        Dsp { dsp }
    }
}

#[derive(Default)]
struct Voice {
    age: usize,
    position: f32,
    increment: f32,
    track_id: Option<usize>,
    insert: Dsp<effects::insert>,
}
impl Voice {
    fn process(&mut self, track: &Track, input: &[f32], output: &mut [f32]) {
        let mut pre_buffer = [0.; effects::insert::INPUTS];
        let mut post_buffer = [0.; effects::insert::OUTPUTS];
        match track.sample_type.load() {
            SampleType::File => {
                let duration = track.file_sample.duration();
                let position = self.position.floor() as usize;
                let position_rem = self.position.fract();
                if position < duration && position_rem == 0. {
                    self.write(&mut pre_buffer, |channel| {
                        track.file_sample.at(position, channel)
                    });
                } else if position + 1 < duration {
                    self.write(&mut pre_buffer, |channel| {
                        let a = track.file_sample.at(position, channel);
                        let b = track.file_sample.at(position + 1, channel);
                        a + position_rem * (b - a)
                    });
                }
            }
            SampleType::Live | SampleType::LiveRecord | SampleType::LivePlay => {
                self.write(&mut pre_buffer, |_| input.iter().sum::<f32>());
            }
        }
        self.insert.dsp.compute(
            1,
            &pre_buffer.each_ref().map(std::slice::from_ref),
            &mut post_buffer.each_mut().map(std::slice::from_mut),
        );
        add_assign_each(output, &post_buffer);
    }
    fn write<F: Fn(usize) -> f32>(&mut self, buffer: &mut [f32], f: F) {
        for (channel, sample) in buffer.iter_mut().enumerate() {
            *sample = f(channel);
        }
        self.position += self.increment;
    }
}

enum Setter {
    Toggle(fn(&mut Audio) -> &AtomicCell<bool>),
    Number(fn(&mut Audio, i32)),
    Entry(&'static str),
}

struct Audio {
    controls: Arc<Controls>,
    voices: Vec<Voice>,
    sends: Vec<Box<dyn Send + FaustDsp<T = f32>>>,
    receiver: Receiver<(Setter, i32)>,
}
impl Audio {
    fn key_down(&mut self, track_id: i32, key: i32) {
        let track_id = track_id.clamp(0, TRACK_COUNT - 1) as usize;
        let track = &self.controls.tracks[track_id];
        let note = track.note(self.controls.root.load(), self.controls.scale.load(), key);
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }
        if let Some(voice) = self.voices.iter_mut().max_by_key(|voice| voice.age) {
            voice.age = 0;
            voice.position = 0.;
            voice.increment = 1.;
            voice.track_id = Some(track_id);
            voice.insert.dsp.instance_clear();
        }
        track.insert.store(&"gate", 1.);
        track.insert.store(&"note", note as f32);
        match track.sample_type.load() {
            SampleType::File | SampleType::LivePlay => track.insert.store(&"live", 1.),
            SampleType::Live | SampleType::LiveRecord => track.insert.store(&"live", 0.),
        };
    }
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
                    if let Some((_, step, value)) = self.controls.find(|ui| ui.map.get(&key)) {
                        match *step as i32 {
                            0 => value.store(bool_to_float(value.load() == 0.)),
                            1 => value.store(data as f32),
                            _ => value.nudge(data, *step),
                        }
                    }
                }
            }
        }
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            let mut pre_buffer = [0.; effects::insert::OUTPUTS];
            let mut post_buffer = [0.; 2];
            for voice in self.voices.iter_mut() {
                let track = match voice.track_id {
                    None => continue,
                    Some(track_id) => &self.controls.tracks[track_id],
                };
                track.insert.set_params_on(&mut voice.insert.dsp);
                voice.process(track, input, &mut pre_buffer);
            }
            add_assign_each(output, &pre_buffer);
            for i in 0..self.sends.len() {
                let start_channel = 2 * (i + 1);
                self.controls.sends[i].set_params_on(self.sends[i].as_mut());
                self.sends[i].compute(
                    1,
                    &pre_buffer.each_ref().map(std::slice::from_ref)[start_channel..],
                    &mut post_buffer.each_mut().map(std::slice::from_mut),
                );
                add_assign_each(output, &post_buffer);
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "context")]
enum Rpc {
    Get { method: String },
    Set { method: String, data: i32 },
}

struct Ui {
    controls: Arc<Controls>,
    sender: Sender<(Setter, i32)>,
}
impl Ui {
    fn process(&self, request: RpcRequest) -> Option<RpcResponse> {
        let (rpc,) = serde_json::from_value(request.params?).ok()?;
        match rpc {
            Rpc::Get { method } => Some(RpcResponse::new_result(
                request.id,
                Some(self.get(&method).into()),
            )),
            Rpc::Set { method, data } => {
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
            "sample:type" => self.controls.active_track().sample_type.load() as i32,
            _ => self
                .controls
                .find(|ui| ui.map.get(method))
                .map_or(0, |(_, _, value)| value.load() as i32),
        }
    }
    fn set(&self, method: &str) -> Setter {
        match method {
            "play" => Setter::Toggle(|audio| &audio.controls.playing),
            "arm" => Setter::Toggle(|audio| &audio.controls.armed),
            "activeTrack" => Setter::Number(|audio, i| audio.controls.active_track_id.store(i)),
            "auditionDown" => Setter::Number(|audio, i| audio.key_down(i, 7)),
            "tempo" => Setter::Number(|audio, i| audio.controls.tempo.nudge(i, 10)),
            "tempoTaps" => Setter::Number(|audio, i| audio.controls.tempo.store(i)),
            "root" => Setter::Number(|audio, i| audio.controls.root.nudge(i, 7)),
            "scale" => Setter::Number(|audio, i| audio.controls.scale.store(i)),
            "noteDown" => Setter::Number(|audio, i| {
                audio.key_down(audio.controls.active_track_id.load(), i);
            }),
            "sample:type" => Setter::Number(|audio, i| {
                audio.controls.active_track().sample_type.store(i.into());
            }),
            _ => Setter::Entry(
                self.controls
                    .find(|ui| ui.map.get_key_value(method).map(|(key, _)| key))
                    .map_or(&"", |key| *key),
            ),
        }
    }
}

fn main() -> Result<()> {
    let mut controls = Controls::default();
    controls.tempo.store(120);
    for i in 0..TRACK_COUNT {
        let mut insert = EffectUi::default();
        effects::insert::build_user_interface_static(&mut insert);
        controls.tracks.push(Track {
            file_sample: Sample::read_file(i)?,
            sample_type: SampleType::File.into(),
            octave: bounded::Int { atom: 4.into() },
            use_key: true.into(),
            insert,
        });
    }

    let sends: Vec<Box<dyn Send + FaustDsp<T = f32>>> = vec![
        Box::new(Dsp::<effects::reverb>::default().dsp),
        Box::new(Dsp::<effects::echo>::default().dsp),
        Box::new(Dsp::<effects::drive>::default().dsp),
    ];
    for dsp in sends.iter() {
        let mut ui = EffectUi::default();
        dsp.build_user_interface(&mut ui);
        controls.sends.push(ui);
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let ui = Ui {
        controls: Arc::new(controls),
        sender,
    };
    let mut audio = Audio {
        controls: Arc::clone(&ui.controls),
        voices: Vec::new(),
        sends,
        receiver,
    };
    for _ in 0..VOICE_COUNT {
        let voice = Voice::default();
        assert_eq!(voice.insert.dsp.get_num_inputs(), 2);
        assert_eq!(
            audio.sends.len() + 1,
            voice.insert.dsp.get_num_outputs() as usize / 2
        );
        audio.voices.push(voice)
    }

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
