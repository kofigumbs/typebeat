#![feature(never_type)]
#![feature(array_methods)]

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};

use bounded::Bounded;
use effects::{FaustDsp, ParamIndex, UI};

mod bounded;
mod effects;
mod samples;
mod ui;

const SEND_COUNT: usize = 3;
const INSERT_OUTPUT_COUNT: usize = 2 + 2 * SEND_COUNT;

const KEY_COUNT: i32 = 15;
const VOICE_COUNT: i32 = 5;
const TRACK_COUNT: i32 = 15;
const SAMPLE_RATE: i32 = 44100;

const SCALE_OFFSETS: &[&[i32]] = &[
    &[0, 2, 4, 5, 7, 9, 11],
    &[0, 2, 3, 5, 7, 8, 10],
    &[0, 2, 3, 5, 7, 8, 11],
    &[0, 2, 3, 5, 7, 9, 11],
];

fn get_clamped<T: Copy>(values: &[T], index: usize) -> T {
    values[index.clamp(0, values.len() - 1)]
}

fn bool_to_float(value: bool) -> f32 {
    return if value { 1. } else { 0. };
}

fn toggle(value: &AtomicCell<bool>) {
    value.fetch_xor(true);
}

#[derive(Clone, Copy, PartialEq)]
enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}
impl SampleType {
    const ALL: [Self; 4] = [
        SampleType::File,
        SampleType::Live,
        SampleType::LiveRecord,
        SampleType::LivePlay,
    ];
    fn thru(self) -> bool {
        match self {
            Self::File | Self::LivePlay => false,
            Self::Live | Self::LiveRecord => true,
        }
    }
}

#[derive(Default)]
struct Entries<T = f32> {
    map: HashMap<&'static str, (ParamIndex, T, bounded::Any<T>)>,
}
impl<T> UI<T> for Entries<T> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, value: T, min: T, max: T, step: T) {
        let entry = bounded::Any {
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
        for (key, (index, _, value)) in self.map.iter() {
            match *key {
                "gate" | "note" => {}
                _ => dsp.set_param(*index, value.load()),
            }
        }
    }
}

struct Track {
    muted: AtomicCell<bool>,
    file_sample: Vec<f32>,
    sample_type: AtomicCell<SampleType>,
    octave: bounded::I32<2, 8>,
    use_key: AtomicCell<bool>,
    active_key: bounded::I32<0, { KEY_COUNT - 1 }>,
    insert: Entries,
}

#[derive(Default)]
struct Controls {
    active_track_id: bounded::I32<0, { TRACK_COUNT - 1 }>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: bounded::I32<1, 999>,
    root: bounded::I32<-12, 12>,
    scale: bounded::I32<0, 4>,
    tracks: Vec<Track>,
    sends: Vec<Entries>,
}
impl Controls {
    fn track(&self, id: i32) -> &Track {
        &self.tracks[id.clamp(0, TRACK_COUNT - 1) as usize]
    }
    fn active_track(&self) -> &Track {
        self.track(self.active_track_id.load())
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
    fn find(&self, key: &str) -> Option<(&&'static str, &(ParamIndex, f32, bounded::Any<f32>))> {
        self.entries()
            .find_map(|entries| entries.map.get_key_value(key))
    }
}

#[derive(Clone)]
struct Live {
    in_use: usize,
    sample: Vec<f32>,
}

struct DspFactory<T> {
    dsp: Box<T>,
}
impl<T: FaustDsp> Default for DspFactory<T> {
    fn default() -> Self {
        let mut dsp = Box::new(T::new());
        dsp.instance_init(SAMPLE_RATE);
        DspFactory { dsp }
    }
}

struct Buffer<const N: usize, const M: usize> {
    mix: [f32; N],
    out: [f32; M],
    mix_start: usize,
}
impl<const N: usize, const M: usize> Buffer<N, M> {
    fn new() -> Self {
        Buffer {
            mix: [0.; N],
            out: [0.; M],
            mix_start: 0,
        }
    }
    fn compute(&mut self, dsp: &mut dyn effects::FaustDsp<T = f32>) {
        dsp.compute(
            1,
            &self.mix.each_ref().map(std::slice::from_ref)[self.mix_start..],
            &mut self.out.each_mut().map(std::slice::from_mut),
        );
    }
    fn write_to(&self, destination: &mut [f32]) {
        for (destination, source) in destination.iter_mut().zip(&self.mix) {
            *destination += source;
        }
    }
}

#[derive(Default)]
struct Voice {
    key: i32,
    age: usize,
    position: f32,
    increment: f32,
    track_id: Option<i32>,
    insert: DspFactory<effects::insert>,
}
impl Voice {
    fn process(&mut self, controls: &Controls, live: &mut Live, input: &[f32], output: &mut [f32]) {
        let mut buffer = Buffer::<2, INSERT_OUTPUT_COUNT>::new();
        match self.track_id {
            None => self.play(&mut buffer.mix, |_| 0.),
            Some(track_id) => {
                let track = controls.track(track_id);
                match track.sample_type.load() {
                    SampleType::File => self.play_sample(&mut buffer.mix, &track.file_sample, 2),
                    SampleType::Live => self.play_thru(&mut buffer.mix, live, input, false),
                    SampleType::LiveRecord => self.play_thru(&mut buffer.mix, live, input, true),
                    SampleType::LivePlay => {
                        self.play_sample(&mut buffer.mix, &live.sample[..live.in_use], 1)
                    }
                }
            }
        }
        buffer.compute(self.insert.dsp.as_mut());
        buffer.write_to(output);
    }
    fn play_thru(&mut self, buffer: &mut [f32], live: &mut Live, input: &[f32], record: bool) {
        let input = input.iter().sum();
        if record && live.in_use < live.sample.len() {
            live.sample[live.in_use] = input;
            live.in_use += 1;
        }
        self.play(buffer, |_| input);
    }
    fn play_sample(&mut self, buffer: &mut [f32], sample: &[f32], channel_count: usize) {
        let position = self.position.floor() as usize;
        let position_fract = self.position.fract();
        self.play(buffer, |channel| {
            let index = |i| i * channel_count + channel % channel_count;
            if position_fract == 0. {
                *sample.get(index(position)).unwrap_or(&0.)
            } else {
                let a = *sample.get(index(position)).unwrap_or(&0.);
                let b = *sample.get(index(position + 1)).unwrap_or(&0.);
                position_fract.mul_add(b - a, a)
            }
        });
    }
    fn play(&mut self, buffer: &mut [f32], f: impl Fn(usize) -> f32) {
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
    lives: Vec<Live>,
    voices: Vec<Voice>,
    sends: Vec<Box<dyn Send + FaustDsp<T = f32>>>,
    receiver: Receiver<Message>,
}
impl Audio {
    fn key_down(&mut self, track_id: i32, key: i32) {
        let track = self.controls.track(track_id);
        track.active_key.store(key);
        if track.muted.load() {
            return;
        }
        let note = self.controls.note(track, key);
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }
        if let Some(voice) = self.voices.iter_mut().max_by_key(|voice| voice.age) {
            voice.key = key;
            voice.age = 0;
            voice.position = 0.;
            voice.increment = (note as f32 / 12.).exp2() / (69.0_f32 / 12.).exp2();
            voice.track_id = Some(track_id);
            voice.insert.dsp.instance_clear();
        }
        track.insert.store(&"gate", 1.);
        track.insert.store(&"note", note as f32);
        track
            .insert
            .store(&"thru", bool_to_float(track.sample_type.load().thru()));
    }
    fn key_up(&mut self, track_id: i32, key: i32) {
        self.release(track_id, |voice, gate| {
            if voice.key == key {
                voice.insert.dsp.set_param(gate, 0.);
            }
        });
    }
    fn set_sample_type(&mut self, value: i32) {
        let track_id = self.controls.active_track_id.load();
        let old = self.controls.active_track().sample_type.load();
        let new = get_clamped(&SampleType::ALL, value as usize);
        if new == SampleType::LiveRecord {
            self.lives[track_id as usize].in_use = 0;
        }
        if old != new {
            self.release(track_id, |voice, gate| {
                voice.insert.dsp.set_param(gate, 0.);
                if !old.thru() && new.thru() {
                    voice.track_id = None;
                }
            });
            self.controls.active_track().sample_type.store(new);
        }
    }
    fn release(&mut self, track_id: i32, f: impl Fn(&mut Voice, ParamIndex)) {
        let track = self.controls.track(track_id);
        if let Some((index, _, _)) = track.insert.map.get(&"gate") {
            self.voices
                .iter_mut()
                .filter(|voice| voice.track_id == Some(track_id))
                .for_each(|voice| f(voice, *index));
        }
    }
    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        while let Ok(setter) = self.receiver.try_recv() {
            match setter {
                Message::Setter(data, f) => f(self, data),
                Message::SetEntry(data, key) => {
                    if let Some((_, (_, step, value))) = self.controls.find(&key) {
                        match *step as i32 {
                            0 => value.store(bool_to_float(value.load() == 0.)),
                            1 => value.store(data as f32),
                            _ => value.nudge(data, *step),
                        }
                    }
                }
            }
        }
        for voice in self.voices.iter_mut() {
            if let Some(track_id) = voice.track_id {
                self.controls
                    .track(track_id)
                    .insert
                    .set_params_on(voice.insert.dsp.as_mut());
            }
        }
        for (dsp, entries) in self.sends.iter_mut().zip(self.controls.sends.iter()) {
            entries.set_params_on(dsp.as_mut());
        }
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            let mut buffer = Buffer::<INSERT_OUTPUT_COUNT, 2>::new();
            for (voice, live) in self.voices.iter_mut().zip(self.lives.iter_mut()) {
                voice.process(&self.controls, live, input, &mut buffer.mix);
            }
            buffer.write_to(output);
            for dsp in self.sends.iter_mut() {
                buffer.mix_start += 2;
                buffer.compute(dsp.as_mut());
                buffer.write_to(output);
            }
        }
    }
}

struct Rpc {
    controls: Arc<Controls>,
    sender: Sender<Message>,
}
impl Rpc {
    fn process(&self, context: &str, method: &str, data: i32) -> Option<i32> {
        let send = |setter| self.send(Message::Setter(data, setter));
        Some(match format!("{} {}", context, method).as_str() {
            "set auditionDown" => {
                send(|audio, i| audio.key_down(i, audio.controls.active_track().active_key.load()))?
            }
            "set auditionUp" => {
                send(|audio, i| audio.key_up(i, audio.controls.active_track().active_key.load()))?
            }
            "set noteDown" => {
                send(|audio, i| audio.key_down(audio.controls.active_track_id.load(), i))?
            }
            "set noteUp" => {
                send(|audio, i| audio.key_up(audio.controls.active_track_id.load(), i))?
            }
            "get activeTrack" => self.controls.active_track_id.load(),
            "set activeTrack" => send(|audio, i| audio.controls.active_track_id.store(i))?,
            "get sample type" => self.controls.active_track().sample_type.load() as i32,
            "set sample type" => send(|audio, i| audio.set_sample_type(i))?,
            "get octave" => self.controls.active_track().octave.load(),
            "set octave" => send(|audio, i| audio.controls.active_track().octave.nudge(i, 0))?,
            "get useKey" => self.controls.active_track().use_key.load().into(),
            "set useKey" => send(|audio, _| toggle(&audio.controls.active_track().use_key))?,
            "get lastKey" => self.controls.active_track().active_key.load(),
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
                } else if let Some((name, i)) = Self::split(method) {
                    match format!("{} {}", context, name).as_str() {
                        "get note" => self.controls.note(self.controls.active_track(), i),
                        "get mute" => self.controls.tracks[i as usize].muted.load() as i32,
                        _ => None?,
                    }
                } else {
                    None?
                }
            }
        })
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
            file_sample: samples::read_stereo_file(i)?,
            sample_type: SampleType::File.into(),
            octave: 4.into(),
            use_key: true.into(),
            active_key: 12.into(),
            insert: Entries::default(),
        };
        effects::insert::build_user_interface_static(&mut track.insert);
        controls.tracks.push(track);
    }

    let sends: Vec<Box<dyn Send + FaustDsp<T = f32>>> = vec![
        DspFactory::<effects::reverb>::default().dsp,
        DspFactory::<effects::echo>::default().dsp,
        DspFactory::<effects::drive>::default().dsp,
    ];
    for dsp in sends.iter() {
        let mut entries = Entries::default();
        dsp.build_user_interface(&mut entries);
        controls.sends.push(entries);
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }
    assert_eq!(sends.len(), SEND_COUNT as usize);

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut audio = Audio {
        controls: Arc::new(controls),
        lives: Vec::new(),
        voices: Vec::new(),
        sends,
        receiver,
    };
    audio.lives.resize_with(TRACK_COUNT as usize, || Live {
        in_use: 0,
        sample: vec![0.; 60 * SAMPLE_RATE as usize],
    });
    for _ in 0..VOICE_COUNT {
        let voice = Voice::default();
        assert_eq!(voice.insert.dsp.get_num_inputs(), 2);
        assert_eq!(
            voice.insert.dsp.get_num_outputs(),
            2 + 2 * audio.sends.len() as i32
        );
        audio.voices.push(voice);
    }

    let rpc = Rpc {
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

    ui::start(move |method, context, data| rpc.process(method, context, data))?;
}
