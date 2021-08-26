#![feature(never_type)]
#![feature(array_methods)]

use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Error, Result};
use crossbeam::atomic::AtomicCell;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::Value;

use effects::{FaustDsp, ParamIndex, UI};
use state::{Enum, Key, State};

mod effects;
mod samples;
mod state;
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

// Song keys
static ACTIVE_TRACK_ID: Key<i32> = Key::new("activeTrack");
static TEMPO: Key<i32> = Key::new("tempo");
static ROOT: Key<i32> = Key::new("root");
static SCALE: Key<i32> = Key::new("scale");

// Track keys
static MUTED: Key<bool> = Key::new("muted");
static USE_KEY: Key<bool> = Key::new("useKey");
static ACTIVE_KEY: Key<i32> = Key::new("activeKey");
static OCTAVE: Key<i32> = Key::new("octave");
static LENGTH: Key<i32> = Key::new("length");
static RESOLUTION: Key<i32> = Key::new("resolution");
static SAMPLE_TYPE: Key<SampleType> = Key::new("sampleType");

// Insert keys -- must match definitions in effects/insert.dsp
static THRU: Key<bool> = Key::new("thru");
static GATE: Key<i32> = Key::new("gate");
static NOTE: Key<i32> = Key::new("note");

fn get_clamped<T>(values: &[T], index: i32) -> &T {
    &values[(index as usize).clamp(0, values.len() - 1)]
}

fn toggle(value: &AtomicCell<bool>) {
    value.fetch_xor(true);
}

struct InitUi<'a> {
    state: &'a mut State<ParamIndex>,
    save: &'a Value,
}

impl<'a> UI<f32> for InitUi<'a> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, n: f32, lo: f32, hi: f32, by: f32) {
        let key = Key::new(s);
        self.state
            .init(self.save, key.between(lo, hi).nudge_by(by as u8), n);
        self.state.with_aux(&Key::<()>::new(s), i);
    }
}

struct SyncUi<'a, T: ?Sized, U> {
    dsp: &'a mut T,
    state: &'a State<U>,
}

impl<'a, T: FaustDsp<T = f32> + ?Sized, U> UI<f32> for SyncUi<'a, T, U> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.dsp.set_param(i, self.state.get(&Key::new(s)));
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}

impl Enum for SampleType {
    const ALL: &'static [Self] = &[Self::File, Self::Live, Self::LiveRecord, Self::LivePlay];
}

impl SampleType {
    fn thru(self) -> bool {
        match self {
            Self::File | Self::LivePlay => false,
            Self::Live | Self::LiveRecord => true,
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
    state: State<ParamIndex>,
    file_sample: Vec<f32>,
    live_sample: Mutex<Vec<f32>>,
    page_start: AtomicCell<i32>,
    sequence: Vec<Step>,
    last_played: [AtomicCell<i32>; KEY_COUNT as usize],
}

impl Default for Track {
    fn default() -> Self {
        Self {
            state: State::default(),
            file_sample: Vec::new(),
            live_sample: Vec::with_capacity(60 * SAMPLE_RATE as usize).into(),
            page_start: 0.into(),
            sequence: vec![Step::default(); MAX_LENGTH as usize],
            last_played: Default::default(),
        }
    }
}

impl Serialize for Track {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.state.serialize(serializer)
    }
}

impl Track {
    fn view_start(&self) -> i32 {
        self.page_start.load() / self.view_length()
    }

    fn view_length(&self) -> i32 {
        MAX_RESOLUTION / self.state.get(&RESOLUTION)
    }

    fn view_from(&self, start: i32) -> View {
        if start >= self.state.get(&LENGTH) {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in start..(start + self.view_length()) {
            let step = get_clamped(&self.sequence, i);
            let change = get_clamped(&step.keys, self.state.get(&ACTIVE_KEY));
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
        if self.state.get(&RESOLUTION) > 1 {
            self.state.set(&RESOLUTION, self.state.get(&RESOLUTION) / 2);
            self.page_start
                .store(self.page_start.load() / self.view_length() * self.view_length());
        }
    }

    fn zoom_in(&self) {
        if self.state.get(&RESOLUTION) < MAX_RESOLUTION {
            self.state.set(&RESOLUTION, self.state.get(&RESOLUTION) * 2);
        }
    }

    fn adjust_page(&self, diff: i32) {
        let new_page_start = self.page_start.load() + diff * VIEWS_PER_PAGE * self.view_length();
        if new_page_start < self.state.get(&LENGTH) {
            self.page_start.store(new_page_start);
        }
    }

    fn adjust_length(&self, diff: i32) {
        self.state
            .set(&LENGTH, self.state.get(&LENGTH) + diff * MAX_RESOLUTION);
    }

    fn toggle_step(&self, i: i32) {
        let start = self.view_index_to_start(i);
        match self.view_from(start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let change =
                    &self.sequence[start as usize].keys[self.state.get(&ACTIVE_KEY) as usize];
                toggle(&change.active);
                change.skip_next.store(false);
                change.value.store(self.view_length());
            }
            View::ContainsSteps => {
                for i in start..(start + self.view_length()) {
                    self.sequence[i as usize].keys[self.state.get(&ACTIVE_KEY) as usize]
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

enum StateId {
    Song,
    ActiveTrack,
}

#[derive(Default)]
struct Controls {
    song: State<ParamIndex>,
    playing: AtomicCell<bool>,
    armed: AtomicCell<bool>,
    step: AtomicCell<i32>,
    frames_since_last_step: AtomicCell<i32>,
    tracks: [Track; TRACK_COUNT as usize],
}

impl Serialize for Controls {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Controls", 2)?;
        s.serialize_field("song", &self.song)?;
        s.serialize_field("tracks", &self.tracks)?;
        s.end()
    }
}

impl Controls {
    fn track(&self, id: i32) -> &Track {
        get_clamped(&self.tracks, id)
    }

    fn active_track(&self) -> &Track {
        self.track(self.song.get(&ACTIVE_TRACK_ID))
    }

    fn toggle_play(&self) {
        toggle(&self.playing);
        self.step.store(0);
        self.frames_since_last_step.store(0);
    }

    fn note(&self, track: &Track, key: i32) -> i32 {
        let root = self.song.get(&ROOT);
        let scale = self.song.get(&SCALE);
        (track.state.get(&OCTAVE) + key / 7) * 12
            + if track.state.get(&USE_KEY) {
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
        SAMPLE_RATE as f32 * 240. / (self.song.get(&TEMPO) as f32) / (resolution as f32)
    }

    fn find_state(&self, name: &str) -> Option<(StateId, Key<i32>)> {
        Some(StateId::Song)
            .zip(self.song.get_key(name))
            .or_else(|| Some(StateId::ActiveTrack).zip(self.active_track().state.get_key(name)))
    }

    fn get_state(&self, id: StateId) -> &State<ParamIndex> {
        match id {
            StateId::Song => &self.song,
            StateId::ActiveTrack => &self.active_track().state,
        }
    }
}

/// Wrapper for FaustDsp that keeps track of its `build_user_interface` fn
struct DspDyn {
    dsp: Box<dyn Send + FaustDsp<T = f32>>,
    builder: fn(&mut dyn UI<f32>),
}

/// Wrapper for FaustDsp that implements Default
struct DspDefault<T> {
    dsp: Box<T>,
}

impl<T: FaustDsp> Default for DspDefault<T> {
    fn default() -> Self {
        let mut dsp = Box::new(T::new());
        dsp.instance_init(SAMPLE_RATE);
        DspDefault { dsp }
    }
}

impl<T: 'static + Send + FaustDsp<T = f32>> DspDefault<T> {
    fn to_dyn(self) -> DspDyn {
        DspDyn {
            dsp: self.dsp,
            builder: T::build_user_interface_static,
        }
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
    insert: DspDefault<effects::insert>,
}

impl Voice {
    // 0 is the "highest" -- voices with priority 0 should not be stolen
    fn priority(&self, controls: &Controls) -> usize {
        match self.track_id {
            Some(track_id) => {
                let track = controls.track(track_id);
                if track.state.get(&SAMPLE_TYPE).thru() {
                    0
                } else {
                    2 - track.state.get(&GATE) as usize
                }
            }
            None => 4,
        }
    }

    fn process(&mut self, controls: &Controls, input: &[f32], output: &mut [f32]) {
        let mut buffer = Buffer::<2, INSERT_OUTPUT_COUNT>::new();
        match self.track_id {
            None => self.play(&mut buffer.mix, |_| 0.),
            Some(track_id) => {
                let track = controls.track(track_id);
                let live = &mut track.live_sample.try_lock().unwrap();
                match track.state.get(&SAMPLE_TYPE) {
                    SampleType::File => self.play_sample(&mut buffer.mix, &track.file_sample, 2),
                    SampleType::Live => self.play_thru(&mut buffer.mix, live, input, false),
                    SampleType::LiveRecord => self.play_thru(&mut buffer.mix, live, input, true),
                    SampleType::LivePlay => self.play_sample(&mut buffer.mix, live, 1),
                }
            }
        }
        buffer.compute(self.insert.dsp.as_mut());
        buffer.write_to(output);
    }

    fn play_thru(&mut self, buffer: &mut [f32], live: &mut Vec<f32>, input: &[f32], record: bool) {
        let input = input.iter().sum();
        if record && live.len() < live.capacity() {
            live.push(input);
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
    SetEntry(i32, StateId, Key<i32>),
}

struct Audio {
    controls: Arc<Controls>,
    voices: Vec<Voice>,
    sends: [DspDyn; SEND_COUNT],
    receiver: Receiver<Message>,
}

impl Audio {
    fn key_down(&mut self, track_id: Option<i32>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.controls.song.get(&ACTIVE_TRACK_ID));
        let track = self.controls.track(track_id);
        let key = key.unwrap_or(track.state.get(&ACTIVE_KEY));
        let song_step = self.controls.step.load();
        if self.controls.playing.load() && self.controls.armed.load() {
            let quantized_step = self
                .controls
                .quantized_step(song_step, track.state.get(&RESOLUTION));
            let track_step =
                get_clamped(&track.sequence, quantized_step % track.state.get(&LENGTH));
            let change = get_clamped(&track_step.keys, key);
            change.active.store(true);
            change.skip_next.store(quantized_step > song_step);
        }
        track.state.set(&ACTIVE_KEY, key);
        self.allocate(track_id, key);
    }

    fn key_up(&mut self, track_id: Option<i32>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.controls.song.get(&ACTIVE_TRACK_ID));
        let track = self.controls.track(track_id);
        let key = key.unwrap_or(track.state.get(&ACTIVE_KEY));
        if self.controls.playing.load() && self.controls.armed.load() {
            let last_played = get_clamped(&track.last_played, key).load();
            let quantized_step = self
                .controls
                .quantized_step(last_played, track.state.get(&RESOLUTION));
            let step = get_clamped(&track.sequence, quantized_step % track.state.get(&LENGTH));
            let change = get_clamped(&step.keys, key);
            change.value.store(self.controls.step.load() - last_played);
        }
        self.release(track_id, key);
    }

    fn set_sample_type(&mut self, value: i32) {
        // Clone for more convenient ownership rules
        let controls = Arc::clone(&self.controls);

        let track = controls.active_track();
        let old = track.state.get(&SAMPLE_TYPE);
        let new = *get_clamped(&SampleType::ALL, value);
        if new == SampleType::LiveRecord {
            track.live_sample.try_lock().unwrap().clear();
        }
        if old != new {
            controls.active_track().state.set(&SAMPLE_TYPE, new);
            self.each_voice_for(controls.song.get(&ACTIVE_TRACK_ID), |voice, gate| {
                voice.track_id = None;
                voice.insert.dsp.set_param(gate, 0.);
            });
            if new.thru() {
                self.allocate(
                    controls.song.get(&ACTIVE_TRACK_ID),
                    track.state.get(&ACTIVE_KEY),
                );
            }
        }
    }

    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        // Process messages for the RPC queue
        while let Ok(setter) = self.receiver.try_recv() {
            match setter {
                Message::Setter(data, f) => f(self, data),
                Message::SetEntry(data, state_id, key) => {
                    self.controls.get_state(state_id).update(&key, data)
                }
            }
        }

        // Update DSP parameters
        for voice in self.voices.iter_mut() {
            if let Some(track_id) = voice.track_id {
                let dsp = voice.insert.dsp.as_mut();
                let state = &self.controls.track(track_id).state;
                effects::insert::build_user_interface_static(&mut SyncUi { dsp, state });
            }
        }
        for DspDyn { dsp, builder } in self.sends.iter_mut() {
            let dsp = dsp.as_mut();
            let state = &self.controls.song;
            builder(&mut SyncUi { dsp, state });
        }

        // Read input frames and calculate output frames
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            // Clone for more convenient ownership rules
            let controls = Arc::clone(&self.controls);

            // If this is a new step, then replay any sequenced events
            if controls.playing.load() && controls.frames_since_last_step.load() == 0 {
                for (track_id, track) in controls.tracks.iter().enumerate() {
                    let length = controls.tracks[track_id].state.get(&LENGTH) as usize;
                    let position = controls.step.load();
                    let step = &track.sequence[position as usize % length];
                    for (key, change) in step.keys.iter().enumerate() {
                        if position - track.last_played[key].load() == change.value.load() {
                            self.release(track_id as i32, key as i32);
                        }
                        if change.skip_next.load() {
                            change.skip_next.store(false);
                        } else if change.active.load() && !track.state.get(&MUTED) {
                            self.allocate(track_id as i32, key as i32);
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
                    controls.step.store(controls.step.load() + 1);
                }
            }

            // Run voices and sends
            let mut buffer = Buffer::<INSERT_OUTPUT_COUNT, 2>::new();
            for voice in self.voices.iter_mut() {
                voice.process(&self.controls, input, &mut buffer.mix);
            }
            buffer.write_to(output);
            for DspDyn { dsp, .. } in self.sends.iter_mut() {
                buffer.mix_start += 2;
                buffer.compute(dsp.as_mut());
                buffer.write_to(output);
            }
        }
    }

    fn allocate(&mut self, track_id: i32, key: i32) {
        self.release(track_id, key);
        let track = self.controls.track(track_id);
        let note = self.controls.note(track, key);
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }

        // Clone for more convenient ownership rules
        let controls = Arc::clone(&self.controls);

        // Steal voie with highest priority number, breaking ties with age
        if let Some(voice) = self
            .voices
            .iter_mut()
            .max_by_key(|voice| (voice.priority(&controls), voice.age))
        {
            voice.key = key;
            voice.age = 0;
            voice.position = 0.;
            voice.increment = (note as f32 / 12.).exp2() / (69.0_f32 / 12.).exp2();
            voice.track_id = Some(track_id);
            voice.insert.dsp.instance_clear();
        }

        // Update DSP parameters
        track.state.set(&GATE, 1);
        track.state.set(&NOTE, note);
        track.state.set(&THRU, track.state.get(&SAMPLE_TYPE).thru());

        // Remember when this was played to for note length sequencer calculation
        get_clamped(&track.last_played, key).store(self.controls.step.load());
    }

    fn release(&mut self, track_id: i32, key: i32) {
        self.each_voice_for(track_id, |voice, gate| {
            if voice.key == key {
                voice.insert.dsp.set_param(gate, 0.);
            }
        });
    }

    fn each_voice_for(&mut self, track_id: i32, f: impl Fn(&mut Voice, ParamIndex)) {
        let track = self.controls.track(track_id);
        self.voices
            .iter_mut()
            .filter(|voice| voice.track_id == Some(track_id))
            .for_each(|voice| f(voice, *track.state.get_aux(&GATE)));
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
            "set sampleType" => send(|audio, i| audio.set_sample_type(i))?,
            "get step" => self.controls.step.load(),
            "get viewStart" => self.controls.active_track().view_start(),
            "set page" => send(|audio, i| audio.controls.active_track().adjust_page(i))?,
            "get bars" => self.controls.active_track().state.get(&LENGTH) / MAX_RESOLUTION,
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
            "set muted" => send(|audio, i| audio.controls.tracks[i as usize].state.toggle(&MUTED))?,
            _ => {
                if let Some((state_id, key)) = self.controls.find_state(method) {
                    match context {
                        "get" => self.controls.get_state(state_id).get(&key),
                        "set" => self.send(Message::SetEntry(data, state_id, key))?,
                        _ => None?,
                    }
                } else if let Some((name, i)) = Self::split(method) {
                    match format!("{} {}", context, name).as_str() {
                        "get view" => self.controls.active_track().view(i) as i32,
                        "get note" => self.controls.note(self.controls.active_track(), i),
                        "get muted" => self.controls.tracks[i as usize].state.get(&MUTED) as i32,
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

struct Autosave {
    controls: Arc<Controls>,
}
impl Autosave {
    fn start(&self) -> anyhow::Result<!> {
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(".typebeat")?;
            serde_json::to_writer_pretty(file, self.controls.as_ref())?;
        }
    }
}

fn main() -> Result<()> {
    let save = &File::open(Path::new(&".typebeat"))
        .map_err(Error::new)
        .and_then(|path| serde_json::from_reader(BufReader::new(path)).map_err(Error::new))
        .unwrap_or_default();

    let mut controls = Controls::default();
    let song = &mut controls.song;
    song.init(save, ACTIVE_TRACK_ID.between(0, TRACK_COUNT - 1), 0);
    song.init(save, TEMPO.between(0, 999).nudge_by(10), 120);
    song.init(save, ROOT.between(-12, 12).nudge_by(7), 0);
    song.init(save, SCALE.between(0, SCALE_OFFSETS.len() as i32 - 1), 0);
    for (i, track) in controls.tracks.iter_mut().enumerate() {
        let save = &save["tracks"][i];
        let state = &mut track.state;
        state.init(save, &MUTED, false);
        state.init(save, USE_KEY.nudge_by(0), true);
        state.init(save, ACTIVE_KEY.between(0, KEY_COUNT), 12);
        state.init(save, OCTAVE.between(2, 8).nudge_by(2), 4);
        state.init(
            save,
            LENGTH.between(MAX_RESOLUTION, MAX_LENGTH),
            MAX_RESOLUTION,
        );
        state.init(save, RESOLUTION.between(1, MAX_RESOLUTION), 16);
        state.init(save, &SAMPLE_TYPE, SampleType::File);
        track.file_sample = samples::read_stereo_file(i)?;
        effects::insert::build_user_interface_static(&mut InitUi { state, save });
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut audio = Audio {
        controls: Arc::new(controls),
        voices: Vec::new(),
        sends: [
            DspDefault::<effects::reverb>::default().to_dyn(),
            DspDefault::<effects::echo>::default().to_dyn(),
            DspDefault::<effects::drive>::default().to_dyn(),
        ],
        receiver,
    };
    for _ in 0..VOICE_COUNT {
        let voice = Voice::default();
        assert_eq!(voice.insert.dsp.get_num_inputs(), 2);
        assert_eq!(
            voice.insert.dsp.get_num_outputs(),
            INSERT_OUTPUT_COUNT as i32
        );
        audio.voices.push(voice);
    }
    for DspDyn { dsp, builder } in audio.sends.iter_mut() {
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
        let state = &mut Arc::get_mut(&mut audio.controls).unwrap().song;
        builder(&mut InitUi { state, save });
    }

    let rpc = Rpc {
        controls: Arc::clone(&audio.controls),
        sender,
    };

    let autosave = Autosave {
        controls: Arc::clone(&audio.controls),
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

    std::thread::spawn(move || autosave.start());
    ui::start(move |method, context, data| rpc.process(method, context, data))?;
}
