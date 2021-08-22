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

const MAX_RESOLUTION: i32 = 512;
const MAX_LENGTH: i32 = MAX_RESOLUTION * 16 * 8;

const KEY_COUNT: i32 = 15;
const VOICE_COUNT: i32 = 5;
const TRACK_COUNT: i32 = 15;
const VIEWS_PER_PAGE: i32 = 4;
const SAMPLE_RATE: i32 = 44100;

const SCALE_OFFSETS: &[&[i32]] = &[
    &[0, 2, 4, 5, 7, 9, 11],
    &[0, 2, 3, 5, 7, 8, 10],
    &[0, 2, 3, 5, 7, 8, 11],
    &[0, 2, 3, 5, 7, 9, 11],
];

fn get_clamped<T>(values: &[T], index: i32) -> &T {
    &values[(index as usize).clamp(0, values.len() - 1)]
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

#[derive(Default)]
struct Change {
    value: AtomicCell<i32>,
    active: AtomicCell<bool>,
    skip_next: AtomicCell<bool>,
}
impl Clone for Change {
    fn clone(&self) -> Self {
        Change {
            value: self.value.load().into(),
            active: self.active.load().into(),
            skip_next: self.skip_next.load().into(),
        }
    }
}

#[derive(Default, Clone)]
struct Step {
    keys: [Change; KEY_COUNT as usize],
}

#[derive(Debug)]
enum View {
    OutOfBounds,
    Empty,
    ExactlyOnStep,
    ContainsSteps,
}

struct Track {
    muted: AtomicCell<bool>,
    file_sample: Vec<f32>,
    sample_type: AtomicCell<SampleType>,
    octave: bounded::I32<2, 8>,
    use_key: AtomicCell<bool>,
    active_key: bounded::I32<0, { KEY_COUNT - 1 }>,
    length: bounded::I32<MAX_RESOLUTION, MAX_LENGTH>,
    resolution: bounded::I32<1, MAX_RESOLUTION>,
    page_start: bounded::I32<0, { i32::MAX }>,
    sequence: Vec<Step>,
    last_played: Vec<AtomicCell<i32>>,
    insert: Entries,
}
impl Default for Track {
    fn default() -> Self {
        Self {
            muted: false.into(),
            file_sample: Vec::new(),
            sample_type: SampleType::File.into(),
            octave: 4.into(),
            use_key: true.into(),
            active_key: 12.into(),
            length: MAX_RESOLUTION.into(),
            resolution: 16.into(),
            page_start: 0.into(),
            sequence: vec![Step::default(); MAX_LENGTH as usize],
            last_played: (0..KEY_COUNT).map(|_| 0.into()).collect(),
            insert: Entries::default(),
        }
    }
}
impl Track {
    fn view_start(&self) -> i32 {
        self.page_start.load() / self.view_length()
    }
    fn view_length(&self) -> i32 {
        MAX_RESOLUTION / self.resolution.load()
    }
    fn view_from(&self, start: i32) -> View {
        if start >= self.length.load() {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in start..(start + self.view_length()) {
            let step = get_clamped(&self.sequence, i);
            let change = get_clamped(&step.keys, self.active_key.load());
            if change.active.load() {
                active_count += 1;
                last_active = i;
            }
        }
        if active_count == 0 {
            View::Empty
        } else if active_count == 1 && last_active == start {
            View::ExactlyOnStep
        } else {
            View::ContainsSteps
        }
    }
    fn view_index_to_start(&self, i: i32) -> i32 {
        self.page_start.load() + i * self.view_length()
    }
    fn view(&self, i: i32) -> View {
        self.view_from(self.view_index_to_start(i))
    }
    fn zoom_out(&self) {
        if self.resolution.load() > 1 {
            self.resolution.store(self.resolution.load() / 2);
            self.page_start
                .store(self.page_start.load() / self.view_length() * self.view_length());
        }
    }
    fn zoom_in(&self) {
        if self.resolution.load() < MAX_RESOLUTION {
            self.resolution.store(self.resolution.load() * 2);
        }
    }
    fn adjust_page(&self, diff: i32) {
        let new_page_start = self.page_start.load() + diff * VIEWS_PER_PAGE * self.view_length();
        if new_page_start < self.length.load() {
            self.page_start.store(new_page_start);
        }
    }
    fn adjust_length(&self, diff: i32) {
        self.length
            .store(self.length.load() + diff * MAX_RESOLUTION)
    }
    fn toggle_step(&self, i: i32) {
        let start = self.view_index_to_start(i);
        match self.view_from(start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let change = &self.sequence[start as usize].keys[self.active_key.load() as usize];
                toggle(&change.active);
                change.skip_next.store(false);
                change.value.store(self.view_length());
            }
            View::ContainsSteps => {
                for i in start..(start + self.view_length()) {
                    self.sequence[i as usize].keys[self.active_key.load() as usize]
                        .active
                        .store(false);
                }
            }
        }
    }
    fn can_clear(&self) -> bool {
        self.changes().any(|change| change.active.load())
    }
    fn clear(&self) {
        self.changes().for_each(|change| change.active.store(false))
    }
    fn changes(&self) -> impl Iterator<Item = &Change> {
        self.sequence.iter().flat_map(|step| step.keys.iter())
    }
}

#[derive(Default)]
struct Controls {
    active_track_id: bounded::I32<0, { TRACK_COUNT - 1 }>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    tempo: bounded::I32<1, 999>,
    root: bounded::I32<-12, 12>,
    scale: bounded::I32<0, { SCALE_OFFSETS[0].len() as i32 }>,
    song_position: bounded::I32<0, { i32::MAX }>,
    frames_since_last_step: bounded::I32<0, { i32::MAX }>,
    tracks: [Track; TRACK_COUNT as usize],
    sends: Vec<Entries>,
}
impl Controls {
    fn track(&self, id: i32) -> &Track {
        &self.tracks[id.clamp(0, TRACK_COUNT - 1) as usize]
    }
    fn active_track(&self) -> &Track {
        self.track(self.active_track_id.load())
    }
    fn toggle_play(&self) {
        toggle(&self.playing);
        self.song_position.store(0);
        self.frames_since_last_step.store(0);
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
    fn quantized_step(&self, step: i32, resolution: i32) -> i32 {
        let scale = MAX_RESOLUTION / resolution;
        let scaled_step = step / scale * scale;
        let snap_to_next = (step - scaled_step) as f32 * self.step_duration(MAX_RESOLUTION)
            + self.frames_since_last_step.load() as f32
            > self.step_duration(resolution) / 2.;
        scaled_step + scale * (snap_to_next as i32)
    }
    fn step_duration(&self, resolution: i32) -> f32 {
        SAMPLE_RATE as f32 * 240. / (self.tempo.load() as f32) / (resolution as f32)
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
    fn key_down(&mut self, track_id: Option<i32>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.controls.active_track_id.load());
        let track = self.controls.track(track_id);
        let key = key.unwrap_or(track.active_key.load());
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
        get_clamped(&track.last_played, key).store(self.controls.song_position.load());
    }
    fn key_up(&mut self, track_id: Option<i32>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.controls.active_track_id.load());
        let key = key.unwrap_or(self.controls.track(track_id).active_key.load());
        self.release(track_id, |voice, gate| {
            if voice.key == key {
                voice.insert.dsp.set_param(gate, 0.);
            }
        });
    }
    fn set_sample_type(&mut self, value: i32) {
        let track_id = self.controls.active_track_id.load();
        let old = self.controls.active_track().sample_type.load();
        let new = *get_clamped(&SampleType::ALL, value);
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
        // Process control messages
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

        // Update Faust parameters
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

        // Process each audio frame
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            // Since most methods called here take &mut self, it becomes inconvenient to use
            // self.controls directly. Calling Arc::clone is just an atomic increment, so we'll do
            // that instead.
            let controls = Arc::clone(&self.controls);

            // If this is a new step, then replay any sequenced events
            if controls.playing.load() && controls.frames_since_last_step.load() == 0 {
                for (track_id, track) in controls.tracks.iter().enumerate() {
                    let length = controls.tracks[track_id].length.load() as usize;
                    let position = controls.song_position.load();
                    let step = &track.sequence[position as usize % length];
                    for (key, change) in step.keys.iter().enumerate() {
                        if position - track.last_played[key].load() == change.value.load() {
                            self.key_up(Some(track_id as i32), Some(key as i32));
                        }
                        if change.skip_next.load() {
                            change.skip_next.store(false);
                        } else if change.active.load() {
                            self.key_down(Some(track_id as i32), Some(key as i32));
                        }
                    }
                }
            }

            // Advance song position
            if controls.playing.load() {
                let next_step = controls.frames_since_last_step.load() + 1;
                controls.frames_since_last_step.store(next_step);
                if next_step as f32 >= controls.step_duration(MAX_RESOLUTION) {
                    controls.frames_since_last_step.store(0);
                    controls
                        .song_position
                        .store(controls.song_position.load() + 1);
                }
            }

            // Run voices and sends
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
            "set auditionDown" => send(|audio, i| audio.key_down(Some(i), None))?,
            "set auditionUp" => send(|audio, i| audio.key_up(Some(i), None))?,
            "set noteDown" => send(|audio, i| audio.key_down(None, Some(i)))?,
            "set noteUp" => send(|audio, i| audio.key_up(None, Some(i)))?,
            "get activeTrack" => self.controls.active_track_id.load(),
            "set activeTrack" => send(|audio, i| audio.controls.active_track_id.store(i))?,
            "get sample type" => self.controls.active_track().sample_type.load() as i32,
            "set sample type" => send(|audio, i| audio.set_sample_type(i))?,
            "get octave" => self.controls.active_track().octave.load(),
            "set octave" => send(|audio, i| audio.controls.active_track().octave.nudge(i, 0))?,
            "get useKey" => self.controls.active_track().use_key.load().into(),
            "set useKey" => send(|audio, _| toggle(&audio.controls.active_track().use_key))?,
            "get step" => self.controls.song_position.load(),
            "get resolution" => self.controls.active_track().resolution.load(),
            "get viewStart" => self.controls.active_track().view_start(),
            "get lastKey" => self.controls.active_track().active_key.load(),
            "set page" => send(|audio, i| audio.controls.active_track().adjust_page(i))?,
            "get bars" => self.controls.active_track().length.load() / MAX_RESOLUTION,
            "set bars" => send(|audio, i| audio.controls.active_track().adjust_length(i))?,
            "set zoomOut" => send(|audio, _| audio.controls.active_track().zoom_out())?,
            "set zoomIn" => send(|audio, _| audio.controls.active_track().zoom_in())?,
            "set sequence" => send(|audio, i| audio.controls.active_track().toggle_step(i))?,
            "get canClear" => self.controls.active_track().can_clear() as i32,
            "set clear" => send(|audio, _| audio.controls.active_track().clear())?,
            "get playing" => self.controls.playing.load().into(),
            "set play" => send(|audio, _| audio.controls.toggle_play())?,
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
                        "get view" => self.controls.active_track().view(i) as i32,
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
    for (i, track) in controls.tracks.iter_mut().enumerate() {
        track.file_sample = samples::read_stereo_file(i)?;
        effects::insert::build_user_interface_static(&mut track.insert);
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
