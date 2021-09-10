#![feature(never_type)]
#![feature(array_methods)]

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use anyhow::Error;
use miniaudio::{Device, DeviceConfig, DeviceType, Format, Frames, FramesMut};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use atomic_cell::AtomicCell;
use effects::{FaustDsp, ParamIndex, UI};
use state::{Enum, Key, State};
use ui::Handler;

mod atomic_cell;
mod effects;
mod samples;
mod state;
mod ui;

const SEND_COUNT: usize = 3;
const INSERT_OUTPUT_COUNT: usize = 2 + 2 * SEND_COUNT;

const MAX_RESOLUTION: usize = 512;
const MAX_LENGTH: usize = MAX_RESOLUTION * 16 * 8;
const VIEWS_PER_PAGE: usize = 4;

const KEY_COUNT: i32 = 15;
const VOICE_COUNT: i32 = 5;
const SAMPLE_RATE: i32 = 44100;
const TRACK_COUNT: usize = 15;

const SCALE_LENGTH: usize = 7;
const SCALE_OFFSETS: &[[i32; SCALE_LENGTH]] = &[
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
    [0, 2, 3, 5, 7, 9, 11],
];

// Song keys
static TEMPO: Key<i32> = Key::new("tempo");
static ROOT: Key<i32> = Key::new("root");
static SCALE: Key<usize> = Key::new("scale");
static ACTIVE_TRACK_ID: Key<usize> = Key::new("activeTrack");

// Track keys
static MUTED: Key<bool> = Key::new("muted");
static USE_KEY: Key<bool> = Key::new("useKey");
static ACTIVE_KEY: Key<i32> = Key::new("activeKey");
static OCTAVE: Key<i32> = Key::new("octave");
static LENGTH: Key<usize> = Key::new("length");
static RESOLUTION: Key<usize> = Key::new("resolution");
static SAMPLE_DETUNE: Key<f32> = Key::new("sampleDetune"); // registered by dsp
static SAMPLE_TYPE: Key<SampleType> = Key::new("sampleType"); // registered by dsp

fn get_clamped<T>(values: &[T], index: i32) -> &T {
    &values[(index as usize).clamp(0, values.len() - 1)]
}

fn write_over(source: &[f32], destination: &mut [f32]) {
    for (destination, source) in destination.iter_mut().zip(source) {
        *destination += source;
    }
}

fn adjust_usize(lhs: usize, diff: i32, rhs: usize) -> usize {
    if diff.is_positive() {
        lhs + diff as usize * rhs
    } else {
        lhs - diff.abs() as usize * rhs
    }
}

#[derive(Default)]
struct ButtonRegisterUi {
    registry: HashMap<&'static str, ParamIndex>,
}

impl UI<f32> for ButtonRegisterUi {
    fn add_button(&mut self, s: &'static str, id: ParamIndex) {
        self.registry.insert(s, id);
    }
}

struct StateRegisterUi<'a> {
    state: &'a mut State,
}

impl<'a> UI<f32> for StateRegisterUi<'a> {
    fn add_num_entry(&mut self, s: &'static str, _: ParamIndex, n: f32, lo: f32, hi: f32, by: f32) {
        self.state
            .register(Key::new(s).between(lo, hi).nudge_by(by as i32).default(n));
    }
}

struct StateSyncUi<'a, T: ?Sized> {
    dsp: &'a mut T,
    state: &'a State,
}

impl<'a, T: FaustDsp<T = f32> + ?Sized> UI<f32> for StateSyncUi<'a, T> {
    fn add_num_entry(&mut self, s: &'static str, id: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.dsp.set_param(id, self.state.get(&Key::new(s)));
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

#[derive(Clone, Default, Deserialize, Serialize)]
struct Change {
    value: AtomicCell<i32>,
    active: AtomicCell<bool>,
    #[serde(skip)]
    skip_next: AtomicCell<bool>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Step {
    #[serde(default)]
    keys: [Change; KEY_COUNT as usize],
}

impl Step {
    fn has_active(&self) -> bool {
        self.keys.iter().any(|change| change.active.load())
    }
}

#[derive(Deserialize, Serialize)]
struct SaveTrack<'a> {
    #[serde(default, borrow)]
    state: HashMap<&'a str, i32>,
    #[serde(default)]
    live: Vec<u8>,
    #[serde(default)]
    sequence: Vec<(usize, Step)>,
}

enum View {
    OutOfBounds,
    Empty,
    ExactlyOnStep,
    ContainsSteps,
}

struct Track {
    state: State,
    file_sample: Vec<f32>,
    live_sample: Vec<AtomicCell<f32>>,
    live_length: AtomicCell<usize>,
    page_start: AtomicCell<usize>,
    sequence: Vec<Step>,
    last_played: [AtomicCell<usize>; KEY_COUNT as usize],
    recent: AtomicCell<bool>,
}

impl Default for Track {
    fn default() -> Self {
        let mut live_sample = Vec::new();
        live_sample.resize_with(60 * SAMPLE_RATE as usize, Default::default);
        Self {
            state: State::default(),
            file_sample: Vec::new(),
            live_sample,
            live_length: 0.into(),
            page_start: 0.into(),
            sequence: vec![Step::default(); MAX_LENGTH as usize],
            last_played: Default::default(),
            recent: false.into(),
        }
    }
}

impl Serialize for Track {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let save = SaveTrack {
            state: self.state.to_save(),
            live: self
                .live()
                .iter()
                .flat_map(|atom| atom.load().to_le_bytes())
                .collect(),
            sequence: self
                .sequence
                .iter()
                .enumerate()
                .filter_map(|(i, step)| step.has_active().then(|| (i, step.clone())))
                .collect(),
        };
        save.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Track {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let save = SaveTrack::deserialize(deserializer)?;
        let mut track = Track::default();
        track.state = save.state.into();
        track.live_length = (save.live.len() / 4).into();
        for (atom, x) in track.live_sample.iter().zip(save.live.chunks_exact(4)) {
            atom.store(f32::from_le_bytes([x[0], x[1], x[2], x[3]]));
        }
        for (i, step) in save.sequence {
            track.sequence.get_mut(i).map(|s| *s = step);
        }
        Ok(track)
    }
}

impl Track {
    fn bars(&self) -> usize {
        self.state.get(&LENGTH) / MAX_RESOLUTION
    }

    fn view_start(&self) -> usize {
        self.page_start.load() / self.view_length()
    }

    fn view_length(&self) -> usize {
        MAX_RESOLUTION / self.state.get(&RESOLUTION)
    }

    fn view_from(&self, start: usize) -> View {
        if start >= self.state.get(&LENGTH) {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in start..(start + self.view_length()) {
            let step = &self.sequence[i];
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

    fn view_index_to_start(&self, i: usize) -> usize {
        self.page_start.load() + i * self.view_length()
    }

    fn view(&self, i: usize) -> View {
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
        let new_page_start = adjust_usize(
            self.page_start.load(),
            diff,
            VIEWS_PER_PAGE * self.view_length(),
        );
        if new_page_start < self.state.get(&LENGTH) {
            self.page_start.store(new_page_start.max(0));
        }
    }

    fn adjust_length(&self, diff: i32) {
        self.state.set(
            &LENGTH,
            adjust_usize(self.state.get(&LENGTH), diff, MAX_RESOLUTION),
        );
    }

    fn toggle_step(&self, i: usize) {
        let start = self.view_index_to_start(i);
        match self.view_from(start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let change =
                    &self.sequence[start as usize].keys[self.state.get(&ACTIVE_KEY) as usize];
                change.active.toggle();
                change.skip_next.store(false);
                change.value.store(self.view_length() as i32);
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

    fn live(&self) -> &[AtomicCell<f32>] {
        &self.live_sample[..self.live_length.load()]
    }
}

enum StateId {
    Song,
    ActiveTrack,
}

#[derive(Default, Deserialize, Serialize)]
struct Song {
    #[serde(default)]
    state: State,
    #[serde(default)]
    tracks: [Track; TRACK_COUNT],
    #[serde(skip)]
    gate_id: ParamIndex,
    #[serde(skip)]
    note_id: ParamIndex,
    #[serde(skip)]
    playing: AtomicCell<bool>,
    #[serde(skip)]
    armed: AtomicCell<bool>,
    #[serde(skip)]
    step: AtomicCell<usize>,
    #[serde(skip)]
    frames_since_last_step: AtomicCell<usize>,
}

impl Song {
    fn active_track(&self) -> &Track {
        &self.tracks[self.state.get(&ACTIVE_TRACK_ID)]
    }

    fn note(&self, track: &Track, key: i32) -> i32 {
        let note_id = key as usize % SCALE_LENGTH;
        (track.state.get(&OCTAVE) + key / 7) * 12
            + if track.state.get(&USE_KEY) {
                SCALE_OFFSETS[self.state.get(&SCALE)][note_id] + self.state.get(&ROOT)
            } else {
                SCALE_OFFSETS[0][note_id]
            }
    }

    fn quantized_step(&self, step: usize, resolution: usize) -> usize {
        let scale = MAX_RESOLUTION / resolution;
        let scaled_step = step / scale * scale;
        let snap_to_next = (step - scaled_step) as f32 * self.step_duration(MAX_RESOLUTION)
            + self.frames_since_last_step.load() as f32
            > self.step_duration(resolution) / 2.;
        scaled_step + scale * (snap_to_next as usize)
    }

    fn step_duration(&self, resolution: usize) -> f32 {
        SAMPLE_RATE as f32 * 240. / (self.state.get(&TEMPO) as f32) / (resolution as f32)
    }

    fn find_state(&self, name: &str) -> Option<(StateId, Key<i32>)> {
        let find = move |id, state: &State| Some(id).zip(state.get_key(name));
        find(StateId::Song, &self.state)
            .or_else(|| find(StateId::ActiveTrack, &self.active_track().state))
    }

    fn get_state(&self, id: StateId) -> &State {
        match id {
            StateId::Song => &self.state,
            StateId::ActiveTrack => &self.active_track().state,
        }
    }
}

/// Wrapper for FaustDsp that implements Clone and Default
struct DspBox<T> {
    dsp: Box<T>,
}

impl<T: FaustDsp> Clone for DspBox<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T: FaustDsp> Default for DspBox<T> {
    fn default() -> Self {
        let mut dsp = Box::new(T::new());
        dsp.init(SAMPLE_RATE);
        DspBox { dsp }
    }
}

/// Wrapper for FaustDsp that keeps track of its #build_user_interface fn
struct DspDyn {
    dsp: Box<dyn Send + FaustDsp<T = f32>>,
    builder: fn(&mut dyn UI<f32>),
}

impl<T: 'static + Send + FaustDsp<T = f32>> From<DspBox<T>> for DspDyn {
    fn from(dsp_box: DspBox<T>) -> Self {
        Self {
            dsp: dsp_box.dsp,
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
}

#[derive(Clone, Default)]
struct Voice {
    key: i32,
    gate: usize,
    age: usize,
    position: f32,
    increment: f32,
    track_id: Option<usize>,
    insert: DspBox<effects::insert>,
}

impl Voice {
    // 0 is the "highest" -- voices with priority 0 should not be stolen
    fn priority(&self, song: &Song) -> usize {
        match self.track_id {
            Some(track_id) => {
                let track = &song.tracks[track_id];
                if track.state.get(&SAMPLE_TYPE).thru() {
                    0
                } else {
                    2 - self.gate
                }
            }
            None => 4,
        }
    }

    fn process(&mut self, song: &Song, input: &[f32], output: &mut [f32]) {
        let mut buffer = Buffer::<2, INSERT_OUTPUT_COUNT>::new();
        match self.track_id {
            None => self.play(&mut buffer.mix, |_| 0.),
            Some(track_id) => {
                let track = &song.tracks[track_id];
                let mix = &mut buffer.mix;
                match track.state.get(&SAMPLE_TYPE) {
                    SampleType::File => self.play_back(mix, &track.file_sample, f32::to_owned, 2),
                    SampleType::Live => self.play_thru(mix, track, input, false),
                    SampleType::LiveRecord => self.play_thru(mix, track, input, true),
                    SampleType::LivePlay => self.play_back(mix, track.live(), |x| x.load(), 1),
                }
            }
        }
        buffer.compute(self.insert.dsp.as_mut());
        write_over(&buffer.out, output);
    }

    fn play_thru(&mut self, mix: &mut [f32], track: &Track, input: &[f32], record: bool) {
        let input = input.iter().sum();
        let length = track.live_length.load();
        if record && length < track.live_sample.len() {
            track.live_sample[length].store(input);
            track.live_length.store(length + 1);
        }
        self.play(mix, |_| input);
    }

    /// Yes, this is a pun -- I wanted a shorter name, and I couldn't help myself
    fn play_back<T>(&mut self, mix: &mut [f32], sample: &[T], f: fn(&T) -> f32, channels: usize) {
        let position = self.position.floor() as usize;
        let position_fract = self.position.fract();
        self.play(mix, |channel| {
            let index = |i| i * channels + channel % channels;
            if position_fract == 0. {
                sample.get(index(position)).map_or(0., f)
            } else {
                let a = sample.get(index(position)).map_or(0., f);
                let b = sample.get(index(position + 1)).map_or(0., f);
                position_fract.mul_add(b - a, a)
            }
        });
    }

    fn play(&mut self, mix: &mut [f32], f: impl Fn(usize) -> f32) {
        for (channel, sample) in mix.iter_mut().enumerate() {
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
    song: Arc<Song>,
    voices: Vec<Voice>,
    sends: [DspDyn; SEND_COUNT],
    receiver: Receiver<Message>,
}

impl Audio {
    fn key_down(&mut self, track_id: Option<usize>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.song.state.get(&ACTIVE_TRACK_ID));
        let track = &self.song.tracks[track_id];
        let key = key.unwrap_or(track.state.get(&ACTIVE_KEY));
        let song_step = self.song.step.load();
        if self.song.playing.load() && self.song.armed.load() {
            let quantized_step = self
                .song
                .quantized_step(song_step, track.state.get(&RESOLUTION));
            let track_step = &track.sequence[quantized_step % track.state.get(&LENGTH)];
            let change = get_clamped(&track_step.keys, key);
            change.active.store(true);
            change.skip_next.store(quantized_step > song_step);
        }
        track.state.set(&ACTIVE_KEY, key);
        self.allocate(track_id, key);
    }

    fn key_up(&mut self, track_id: Option<usize>, key: Option<i32>) {
        let track_id = track_id.unwrap_or(self.song.state.get(&ACTIVE_TRACK_ID));
        let track = &self.song.tracks[track_id];
        let key = key.unwrap_or(track.state.get(&ACTIVE_KEY));
        if self.song.playing.load() && self.song.armed.load() {
            let last_played = get_clamped(&track.last_played, key).load();
            let quantized_step = self
                .song
                .quantized_step(last_played, track.state.get(&RESOLUTION));
            let step = &track.sequence[quantized_step % track.state.get(&LENGTH)];
            get_clamped(&step.keys, key)
                .value
                .store((self.song.step.load() - last_played) as i32);
        }
        self.release(track_id, key);
    }

    fn set_sample_type(&mut self, value: i32) {
        // Clone for more convenient ownership rules
        let song = Arc::clone(&self.song);

        let track = song.active_track();
        let old = track.state.get(&SAMPLE_TYPE);
        let new = *get_clamped(&SampleType::ALL, value);
        if new == SampleType::LiveRecord {
            track.live_length.store(0);
        }
        if old != new {
            song.active_track().state.set(&SAMPLE_TYPE, new);
            for voice in self.each_voice_for(song.state.get(&ACTIVE_TRACK_ID)) {
                voice.gate = 0;
                voice.track_id = None;
            }
            if new.thru() {
                self.allocate(
                    song.state.get(&ACTIVE_TRACK_ID),
                    track.state.get(&ACTIVE_KEY),
                );
            }
        }
    }

    fn toggle_play(&mut self) {
        self.song.playing.toggle();
        self.song.step.store(0);
        self.song.frames_since_last_step.store(0);
        if !self.song.playing.load() {
            self.voices.iter_mut().for_each(|voice| voice.gate = 0);
        }
    }

    fn process(&mut self, input: &Frames, output: &mut FramesMut) {
        // Process messages for the Controller queue
        while let Ok(setter) = self.receiver.try_recv() {
            match setter {
                Message::Setter(data, f) => f(self, data),
                Message::SetEntry(data, state_id, key) => {
                    self.song.get_state(state_id).nudge(&key, data)
                }
            }
        }

        // Update DSP parameters
        for voice in self.voices.iter_mut() {
            if let Some(track_id) = voice.track_id {
                let dsp = voice.insert.dsp.as_mut();
                let state = &self.song.tracks[track_id].state;
                dsp.set_param(self.song.gate_id, voice.gate as f32);
                effects::insert::build_user_interface_static(&mut StateSyncUi { dsp, state });
            }
        }
        for DspDyn { dsp, builder } in self.sends.iter_mut() {
            let dsp = dsp.as_mut();
            let state = &self.song.state;
            builder(&mut StateSyncUi { dsp, state });
        }

        // Read input frames and calculate output frames
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            // Clone for more convenient ownership rules
            let song = Arc::clone(&self.song);

            // If this is a new step, then replay any sequenced events
            if song.playing.load() && song.frames_since_last_step.load() == 0 {
                for (track_id, track) in song.tracks.iter().enumerate() {
                    let length = song.tracks[track_id].state.get(&LENGTH);
                    let song_step = song.step.load();
                    let step = &track.sequence[song_step as usize % length];
                    for (key, change) in step.keys.iter().enumerate() {
                        // Check if key should be released per its sequenced duration
                        let duration = song_step - track.last_played[key].load();
                        let start_step = &track.sequence[track.last_played[key].load() % length];
                        if duration as i32 == start_step.keys[key].value.load() {
                            self.release(track_id, key as i32);
                        }

                        // Check if key should be played
                        if change.skip_next.load() {
                            change.skip_next.store(false);
                        } else if change.active.load() && !track.state.get(&MUTED) {
                            self.allocate(track_id, key as i32);
                        }
                    }
                }
            }

            // Advance song step
            if song.playing.load() {
                let next_step = song.frames_since_last_step.load() + 1;
                song.frames_since_last_step.store(next_step);
                if next_step as f32 >= song.step_duration(MAX_RESOLUTION) {
                    song.frames_since_last_step.store(0);
                    song.step.store(song.step.load() + 1);
                }
            }

            // Run voices and sends
            let mut buffer = Buffer::<INSERT_OUTPUT_COUNT, 2>::new();
            for voice in self.voices.iter_mut() {
                voice.process(&self.song, input, &mut buffer.mix);
            }
            write_over(&buffer.mix, output);
            for DspDyn { dsp, .. } in self.sends.iter_mut() {
                buffer.mix_start += 2;
                buffer.compute(dsp.as_mut());
                write_over(&buffer.out, output);
            }
        }
    }

    fn allocate(&mut self, track_id: usize, key: i32) {
        self.release(track_id, key);
        let track = &self.song.tracks[track_id];
        let note = self.song.note(track, key) as f32;
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }

        // Clone for more convenient ownership rules
        let song = Arc::clone(&self.song);

        // Steal voie with highest priority number, breaking ties with age
        if let Some(voice) = self
            .voices
            .iter_mut()
            .max_by_key(|voice| (voice.priority(&song), voice.age))
        {
            voice.key = key;
            voice.gate = 1;
            voice.age = 0;
            voice.position = 0.;
            voice.increment = ((note + track.state.get(&SAMPLE_DETUNE) / 10.) / 12.).exp2()
                / (69.0_f32 / 12.).exp2();
            voice.track_id = Some(track_id);
            voice.insert.dsp.instance_clear();
            voice.insert.dsp.set_param(self.song.note_id, note);
        }

        // Remember when this was played to for note length sequencer calculation
        get_clamped(&track.last_played, key).store(self.song.step.load());

        // Inform UI
        track.recent.store(true);
    }

    fn release(&mut self, track_id: usize, key: i32) {
        self.each_voice_for(track_id)
            .filter(|voice| voice.key == key)
            .for_each(|voice| voice.gate = 0);
    }

    fn each_voice_for(&mut self, track_id: usize) -> impl Iterator<Item = &mut Voice> {
        self.voices
            .iter_mut()
            .filter(move |voice| voice.track_id == Some(track_id))
    }
}

struct Controller {
    song: Arc<Song>,
    sender: Sender<Message>,
}

impl Handler for Controller {
    fn on_save(&self) {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        let file = options.open(".typebeat").unwrap();
        serde_json::to_writer(file, self.song.as_ref()).unwrap();
    }

    fn on_rpc(&self, context: &str, method: &str, data: i32) -> Option<i32> {
        let send = |setter| self.send(Message::Setter(data, setter));
        Some(match format!("{} {}", context, method).as_str() {
            "get armed" => self.song.armed.load().into(),
            "get bars" => self.song.active_track().bars() as i32,
            "get canClear" => self.song.active_track().can_clear() as i32,
            "get playing" => self.song.playing.load().into(),
            "get step" => self.song.step.load() as i32,
            "get viewStart" => self.song.active_track().view_start() as i32,
            "set armed" => send(|audio, _| audio.song.armed.toggle())?,
            "set auditionDown" => send(|audio, i| audio.key_down(Some(i as usize), None))?,
            "set auditionUp" => send(|audio, i| audio.key_up(Some(i as usize), None))?,
            "set bars" => send(|audio, i| audio.song.active_track().adjust_length(i))?,
            "set clear" => send(|audio, _| audio.song.active_track().clear())?,
            "set muted" => send(|audio, i| audio.song.tracks[i as usize].state.toggle(&MUTED))?,
            "set noteDown" => send(|audio, i| audio.key_down(None, Some(i)))?,
            "set noteUp" => send(|audio, i| audio.key_up(None, Some(i)))?,
            "set page" => send(|audio, i| audio.song.active_track().adjust_page(i))?,
            "set playing" => send(|audio, _| audio.toggle_play())?,
            "set sampleType" => send(|audio, i| audio.set_sample_type(i))?,
            "set sequence" => send(|audio, i| audio.song.active_track().toggle_step(i as usize))?,
            "set zoomIn" => send(|audio, _| audio.song.active_track().zoom_in())?,
            "set zoomOut" => send(|audio, _| audio.song.active_track().zoom_out())?,
            _ => {
                if let Some((state_id, key)) = self.song.find_state(method) {
                    match context {
                        "get" => self.song.get_state(state_id).get(&key),
                        "set" => self.send(Message::SetEntry(data, state_id, key))?,
                        _ => None?,
                    }
                } else if let Some((name, i)) = Self::split(method) {
                    match format!("{} {}", context, name).as_str() {
                        "get muted" => self.song.tracks[i as usize].state.get(&MUTED) as i32,
                        "get note" => self.song.note(self.song.active_track(), i),
                        "get recent" => self.song.tracks[i as usize].recent.swap(false) as i32,
                        "get view" => self.song.active_track().view(i as usize) as i32,
                        _ => None?,
                    }
                } else {
                    None?
                }
            }
        })
    }
}

impl Controller {
    fn send<T>(&self, message: Message) -> Option<T> {
        let _ = self.sender.send(message);
        None
    }

    fn split(method: &str) -> Option<(&str, i32)> {
        let mut iter = method.split(' ');
        iter.next().zip(iter.next()?.parse().ok())
    }
}

fn main() -> Result<(), Error> {
    let mut song: Song = std::fs::read(Path::new(&".typebeat"))
        .map_err(Error::new)
        .and_then(|s| serde_json::from_slice(&s).map_err(Error::new))
        .unwrap_or_default();

    let mut buttons = ButtonRegisterUi::default();
    effects::insert::build_user_interface_static(&mut buttons);
    song.gate_id = buttons.registry["gate"];
    song.note_id = buttons.registry["note"];

    let state = &mut song.state;
    state.register(ACTIVE_TRACK_ID.between(0, TRACK_COUNT - 1));
    state.register(TEMPO.between(0, 999).default(120).nudge_by(10));
    state.register(ROOT.between(-12, 12).nudge_by(7));
    state.register(SCALE.between(0, SCALE_OFFSETS.len() - 1));

    for (i, track) in song.tracks.iter_mut().enumerate() {
        let state = &mut track.state;
        state.register(&MUTED);
        state.register(USE_KEY.nudge_by(0).default(true));
        state.register(ACTIVE_KEY.between(0, KEY_COUNT).default(12));
        state.register(OCTAVE.between(2, 8).default(4).nudge_by(2));
        state.register(
            LENGTH
                .between(MAX_RESOLUTION, MAX_LENGTH)
                .default(MAX_RESOLUTION),
        );
        state.register(RESOLUTION.between(1, MAX_RESOLUTION).default(16));
        effects::insert::build_user_interface_static(&mut StateRegisterUi { state });
        track.file_sample = samples::read_stereo_file(i)?;
    }

    let mut sends: [DspDyn; SEND_COUNT] = [
        DspBox::<effects::reverb>::default().into(),
        DspBox::<effects::echo>::default().into(),
        DspBox::<effects::drive>::default().into(),
    ];
    for DspDyn { dsp, builder } in sends.iter_mut() {
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
        builder(&mut StateRegisterUi { state });
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut audio = Audio {
        song: Arc::new(song),
        voices: vec![Voice::default(); VOICE_COUNT as usize],
        sends,
        receiver,
    };
    for voice in audio.voices.iter() {
        assert_eq!(voice.insert.dsp.get_num_inputs(), 2);
        assert_eq!(
            voice.insert.dsp.get_num_outputs(),
            INSERT_OUTPUT_COUNT as i32
        );
    }

    let controller = Controller {
        song: Arc::clone(&audio.song),
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

    ui::start(controller)?;
}
