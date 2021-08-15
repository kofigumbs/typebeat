#![feature(array_methods)]

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};
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

const KEY_COUNT: i32 = 15;
const VOICE_COUNT: i32 = 5;
const TRACK_COUNT: i32 = 15;
const SAMPLE_RATE: i32 = 44100;

const SCALE_OFFSETS: [[i32; 7]; 4] = [
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
    [0, 2, 3, 5, 7, 9, 11],
];

fn add_assign_each(destination: &mut [f32], source: &[f32]) {
    for (destination, source) in destination.iter_mut().zip(source) {
        *destination += source;
    }
}

fn toggle(value: &AtomicCell<bool>) {
    value.fetch_xor(true);
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
struct Entries<T = f32> {
    map: HashMap<&'static str, (ParamIndex, T, bounded::Dynamic<T>)>,
}
impl<T> UI<T> for Entries<T> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, value: T, min: T, max: T, step: T) {
        let entry = bounded::Dynamic {
            atom: value.into(),
            min,
            max,
        };
        self.map.insert(s, (i, step, entry));
    }
}
impl Entries {
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
    muted: AtomicCell<bool>,
    file_sample: Sample,
    sample_type: AtomicCell<SampleType>,
    octave: bounded::Int<2, 8>,
    use_key: AtomicCell<bool>,
    last_key: bounded::Int<0, { KEY_COUNT - 1 }>,
    insert: Entries,
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
    sends: Vec<Entries>,
}
impl Controls {
    fn active_track(&self) -> &Track {
        &self.tracks[self.active_track_id.load() as usize]
    }
    fn note(&self, track: &Track, key: i32) -> i32 {
        let root = self.root.load();
        let scale = self.scale.load();
        (track.octave.load() + key / 7) * 12
            + if track.use_key.load() {
                SCALE_OFFSETS[scale as usize][key as usize % 7] + root
            } else {
                SCALE_OFFSETS[0][key as usize % 7]
            }
    }
    fn entries(&self) -> impl Iterator<Item = &Entries> {
        std::array::from_ref(&self.active_track().insert)
            .iter()
            .chain(self.sends.iter())
    }
    fn find(
        &self,
        key: &str,
    ) -> Option<(&&'static str, &(ParamIndex, f32, bounded::Dynamic<f32>))> {
        self.entries()
            .find_map(|entries| entries.map.get_key_value(key))
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
                let position = self.position.floor() as usize;
                let position_fract = self.position.fract();
                self.write(&mut pre_buffer, |channel| {
                    let a = track.file_sample.at(position, channel);
                    let b = track.file_sample.at(position + 1, channel);
                    position_fract.mul_add(b - a, a)
                });
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

enum Message {
    Setter(i32, fn(&mut Audio, i32)),
    SetEntry(i32, &'static str),
}

struct Audio {
    controls: Arc<Controls>,
    voices: Vec<Voice>,
    sends: Vec<Box<dyn Send + FaustDsp<T = f32>>>,
    receiver: Receiver<Message>,
}
impl Audio {
    fn key_down(&mut self, track_id: i32, key: i32) {
        let track_id = track_id.clamp(0, TRACK_COUNT - 1) as usize;
        let track = &self.controls.tracks[track_id];
        track.last_key.store(key);
        if track.muted.load() {
            return;
        }
        let note = self.controls.note(track, key);
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }
        if let Some(voice) = self.voices.iter_mut().max_by_key(|voice| voice.age) {
            voice.age = 0;
            voice.position = 0.;
            voice.increment = (note as f32 / 12.).exp2() / (69.0_f32 / 12.).exp2();
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
        while let Ok(setter) = self.receiver.try_recv() {
            match setter {
                Message::Setter(data, f) => f(self, data),
                Message::SetEntry(data, key) => {
                    if let Some((_, (_, step, value))) = self.controls.find(&key) {
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

struct Ui {
    controls: Arc<Controls>,
    sender: Sender<Message>,
}
impl Ui {
    fn process(&self, request: RpcRequest) -> Option<RpcResponse> {
        let param = &request.params?[0];
        let context = param["context"].as_str()?;
        let method = param["method"].as_str()?;
        let data = param["data"].as_i64()? as i32;
        let send = |setter| self.send(Message::Setter(data, setter));
        let response = match format!("{} {}", context, method).as_str() {
            "set auditionDown" => {
                send(|audio, i| audio.key_down(i, audio.controls.active_track().last_key.load()))?
            }
            "set noteDown" => {
                send(|audio, i| audio.key_down(audio.controls.active_track_id.load(), i))?
            }
            "get activeTrack" => self.controls.active_track_id.load(),
            "set activeTrack" => send(|audio, i| audio.controls.active_track_id.store(i))?,
            "get sample type" => self.controls.active_track().sample_type.load() as i32,
            "set sample type" => {
                send(|audio, i| audio.controls.active_track().sample_type.store(i.into()))?
            }
            "get octave" => self.controls.active_track().octave.load(),
            "set octave" => send(|audio, i| audio.controls.active_track().octave.nudge(i, 0))?,
            "get useKey" => self.controls.active_track().use_key.load().into(),
            "set useKey" => send(|audio, _| toggle(&audio.controls.active_track().use_key))?,
            "get lastKey" => self.controls.active_track().last_key.load(),
            "get playing" => self.controls.playing.load().into(),
            "set play" => send(|audio, _| toggle(&audio.controls.playing))?,
            "get armed" => self.controls.armed.load().into(),
            "set arm" => send(|audio, _| toggle(&audio.controls.armed))?,
            "get tempo" => self.controls.tempo.load(),
            "set tempo" => send(|audio, i| audio.controls.tempo.nudge(i, 10))?,
            "set tempoTaps" => send(|audio, i| audio.controls.tempo.store(i))?,
            "get root" => self.controls.root.load(),
            "set root" => send(|audio, i| audio.controls.root.nudge(i, 7))?,
            "get scale" => self.controls.scale.load(),
            "set scale" => send(|audio, i| audio.controls.scale.store(i))?,
            "set mute" => send(|audio, i| toggle(&audio.controls.tracks[i as usize].muted))?,
            _ => {
                if let Some((key, (_, _, value))) = self.controls.find(method) {
                    match context {
                        "get" => value.load() as i32,
                        "set" => self.send(Message::SetEntry(data, key))?,
                        _ => None?,
                    }
                } else if let Some((name, i)) = Ui::split(method) {
                    match format!("{} {}", context, name).as_str() {
                        "get note" => self.controls.note(self.controls.active_track(), i),
                        "get mute" => self.controls.tracks[i as usize].muted.load() as i32,
                        _ => None?,
                    }
                } else {
                    None?
                }
            }
        };
        Some(RpcResponse::new_result(request.id, Some(response.into())))
    }
    fn send<T>(&self, message: Message) -> Option<T> {
        let _ = self.sender.send(message);
        None
    }
    fn split(method: &str) -> Option<(&str, i32)> {
        let mut iter = method.split(' ');
        iter.next().zip(iter.next()?.parse().ok())
    }
}

fn main() -> Result<()> {
    let mut controls = Controls::default();
    controls.tempo.store(120);
    for i in 0..TRACK_COUNT {
        let mut track = Track {
            muted: false.into(),
            file_sample: Sample::read_file(i)?,
            sample_type: SampleType::File.into(),
            octave: 4.into(),
            use_key: true.into(),
            last_key: 12.into(),
            insert: Entries::default(),
        };
        effects::insert::build_user_interface_static(&mut track.insert);
        controls.tracks.push(track);
    }

    let sends: Vec<Box<dyn Send + FaustDsp<T = f32>>> = vec![
        Box::new(Dsp::<effects::reverb>::default().dsp),
        Box::new(Dsp::<effects::echo>::default().dsp),
        Box::new(Dsp::<effects::drive>::default().dsp),
    ];
    for dsp in sends.iter() {
        let mut entries = Entries::default();
        dsp.build_user_interface(&mut entries);
        controls.sends.push(entries);
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut audio = Audio {
        controls: Arc::new(controls),
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
    let ui = Ui {
        controls: Arc::clone(&audio.controls),
        sender,
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
