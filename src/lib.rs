#![feature(array_methods)]
#![feature(bool_to_option)]
#![feature(format_args_capture)]

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};

use default_boxed::DefaultBoxed;
use miniaudio::{
    Decoder, DecoderConfig, Device, DeviceConfig, DeviceType, Format, Frames, FramesMut,
};
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use atomic_cell::{AtomicCell, CopyAs};
use effects::{FaustDsp, ParamIndex, UI};
pub use state::Strategy;
use state::{Host, Param, State};

mod atomic_cell;
mod effects;
mod state;

const SEND_COUNT: usize = 3;
const INSERT_OUTPUT_COUNT: usize = 2 + 2 * SEND_COUNT;

const MAX_RES: usize = 512;
const MAX_LENGTH: usize = MAX_RES * 8;
const VIEWS_PER_PAGE: usize = 4;

const KEY_COUNT: i32 = 15;
const SAMPLE_RATE: i32 = 44100;
const TRACK_COUNT: usize = 15;

const SCALE_LENGTH: usize = 7;
const SCALE_OFFSETS: &[[i32; SCALE_LENGTH]] = &[
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
    [0, 2, 3, 5, 7, 9, 11],
];

const DEFAULT_SAMPLES: &[&str] = &[
    "kick.wav",
    "kickme.wav",
    "tom.wav",
    "conga.wav",
    "cabasa.wav",
    "sd.wav",
    "sst.wav",
    "clap.wav",
    "cowb.wav",
    "tamb.wav",
    "chhl.wav",
    "chhs.wav",
    "crash.wav",
    "ride.wav",
    "tag.wav",
];

lazy_static::lazy_static! {
    static ref NOTE: Vec<String> = (0..TRACK_COUNT).map(|i| format!("note{i}")).collect();
    static ref VIEW: Vec<String> = (0..4).map(|i| format!("view{i}")).collect();
    static ref WAVEFORM: Vec<String> = (0..25).map(|i| format!("waveform{i}")).collect();
}

/// Wrapper for FaustDsp that implements Clone and Default
#[derive(Clone)]
struct DspBox<T> {
    dsp: Box<T>,
}

impl<T: FaustDsp + DefaultBoxed> Default for DspBox<T> {
    fn default() -> Self {
        let mut dsp = T::default_boxed();
        dsp.init(SAMPLE_RATE);
        DspBox { dsp }
    }
}

/// Type-erased wrapper for FaustDsp
struct DspDyn {
    dsp: Box<dyn Send + Sync + FaustDsp<T = f32>>,
    builder: fn(&mut dyn UI<f32>),
}

impl<T: 'static + Send + Sync + FaustDsp<T = f32>> From<(&'static str, DspBox<T>)> for DspDyn {
    fn from(value: (&'static str, DspBox<T>)) -> Self {
        Self {
            dsp: value.1.dsp,
            builder: T::build_user_interface_static,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}

impl From<usize> for SampleType {
    fn from(value: usize) -> Self {
        let all = [Self::File, Self::Live, Self::LiveRecord, Self::LivePlay];
        all[value.min(all.len() - 1)]
    }
}

impl AsPrimitive<i32> for SampleType {
    fn as_(self) -> i32 {
        self as i32
    }
}

impl SampleType {
    fn thru(self) -> bool {
        match self {
            Self::File | Self::LivePlay => false,
            Self::Live | Self::LiveRecord => true,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
enum EventType {
    Key(usize),
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Event {
    value: AtomicCell<i32>,
    active: AtomicCell<bool>,
    #[serde(skip)]
    skip_next: AtomicCell<bool>,
}

impl Event {
    fn set(&self, value: i32) {
        self.value.store(value);
        self.active.store(true);
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Step {
    #[serde(default)]
    keys: [Event; KEY_COUNT as usize],
}

#[derive(Clone, Copy, Deserialize_repr, Serialize_repr)]
#[repr(usize)]
pub enum View {
    OutOfBounds,
    Empty,
    ExactlyOnStep,
    ContainsSteps,
}

impl AsPrimitive<i32> for View {
    fn as_(self) -> i32 {
        self as i32
    }
}

struct Track {
    state: State<Track>,
    file_sample: Vec<f32>,
    live_sample: Vec<AtomicCell<f32>>,
    live_length: AtomicCell<usize>,
    sequence: Vec<Step>,
    last_played: [AtomicCell<usize>; KEY_COUNT as usize],
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
            sequence: vec![Step::default(); MAX_LENGTH as usize],
            last_played: Default::default(),
        }
    }
}

impl Host for Track {
    fn for_each_param<F: FnMut(&'static str, &Param)>(f: &mut F) {
        effects::insert::for_each_param(f);
        f("activeKey", Param::new(12).max(TRACK_COUNT - 1));
        f("bars", Param::new(1).temp());
        f("canClear", Param::new(false).temp());
        f("length", Param::new(MAX_RES).min(MAX_RES).max(MAX_RES * 8));
        f("muted", Param::new(false).toggle());
        f("octave", Param::new(4).min(2).max(8).step(2));
        f("pageStart", Param::new(0).temp());
        f("recent", Param::new(0).temp());
        f("resolution", Param::new(16).min(1).max(MAX_RES));
        f("usingKey", Param::new(true).toggle());
        f("viewStart", Param::new(0).temp());
        NOTE.iter().for_each(|name| f(name, Param::new(0).temp()));
        VIEW.iter().for_each(|name| f(name, Param::new(0).temp()));
        WAVEFORM
            .iter()
            .for_each(|name| f(name, Param::new(0).temp()));
    }
}

impl Track {
    fn sample_type(&self) -> SampleType {
        self.state.get::<usize>("sampleType").into()
    }

    fn bars(&self) -> usize {
        self.state.get::<usize>("length") / MAX_RES
    }

    fn view_start(&self) -> usize {
        self.state.get::<usize>("pageStart") / self.view_length()
    }

    fn view_length(&self) -> usize {
        MAX_RES / self.state.get::<usize>("resolution")
    }

    fn view_from(&self, start: usize) -> View {
        if start >= self.state.get("length") {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in start..(start + self.view_length()) {
            let step = &self.sequence[i];
            let event = &step.keys[self.state.get::<usize>("activeKey")];
            if event.active.load() {
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
        self.state.get::<usize>("pageStart") + i * self.view_length()
    }

    fn view(&self, i: usize) -> View {
        self.view_from(self.view_index_to_start(i))
    }

    fn zoom_out(&self) {
        if self.state.get::<usize>("resolution") > 1 {
            self.state
                .set("resolution", self.state.get::<usize>("resolution") / 2);
            self.state.set(
                "pageStart",
                self.state.get::<usize>("pageStart") / self.view_length() * self.view_length(),
            );
        }
    }

    fn zoom_in(&self) {
        if self.state.get::<usize>("resolution") < MAX_RES {
            self.state
                .set("resolution", self.state.get::<usize>("resolution") * 2);
        }
    }

    fn adjust_page(&self, diff: i32) {
        let new_page_start = Track::adjust(
            self.state.get("pageStart"),
            diff,
            VIEWS_PER_PAGE * self.view_length(),
        );
        if new_page_start < self.state.get("length") {
            self.state.set("pageStart", new_page_start.max(0));
        }
    }

    fn adjust_length(&self, diff: i32) {
        self.state.set(
            "length",
            Track::adjust(self.state.get("length"), diff, MAX_RES),
        );
    }

    fn toggle_step(&self, i: usize) {
        let start = self.view_index_to_start(i);
        match self.view_from(start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let event =
                    &self.sequence[start as usize].keys[self.state.get::<usize>("activeKey")];
                event.active.toggle();
                event.skip_next.store(false);
                event.value.store(self.view_length() as i32);
            }
            View::ContainsSteps => {
                for i in start..(start + self.view_length()) {
                    self.sequence[i].keys[self.state.get::<usize>("activeKey")]
                        .active
                        .store(false);
                }
            }
        }
    }

    fn can_clear(&self) -> bool {
        self.events().any(|(_, event)| event.active.load())
    }

    fn clear(&self) {
        self.events()
            .for_each(|(_, event)| event.active.store(false));
    }

    fn events(&self) -> impl Iterator<Item = (EventType, &Event)> {
        self.sequence
            .iter()
            .take(self.state.get("length"))
            .flat_map(|step| (0..).map(EventType::Key).zip(step.keys.iter()))
    }

    fn live(&self) -> &[AtomicCell<f32>] {
        &self.live_sample[..self.live_length.load()]
    }

    fn waveform(&self, i: usize) -> i32 {
        match self.sample_type() {
            SampleType::File => self.sample_waveform(i, &self.file_sample),
            SampleType::LivePlay => self.sample_waveform(i, self.live()),
            SampleType::Live | SampleType::LiveRecord => 0,
        }
    }

    fn adjust(lhs: usize, diff: i32, rhs: usize) -> usize {
        if diff.is_positive() {
            lhs + diff as usize * rhs
        } else {
            lhs - diff.abs() as usize * rhs
        }
    }

    fn sample_waveform<T: CopyAs<f32>>(&self, i: usize, sample: &[T]) -> i32 {
        let chunk_len = sample.len() / WAVEFORM.len();
        let aggregate = sample
            .chunks(chunk_len)
            .nth(i)
            .unwrap_or_default()
            .into_iter()
            .fold(0., |max, f| f.copy_as().abs().max(max));
        (100. * aggregate) as i32
    }
}

pub enum Change {
    Song(&'static str, i32),
    Track(usize, &'static str, i32),
}

impl Serialize for Change {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Change::Song(method, value) => ("song", method, value).serialize(serializer),
            Change::Track(i, method, value) => ("tracks", i, method, value).serialize(serializer),
        }
    }
}

pub struct Platform {
    pub voice_count: usize,
    pub root: PathBuf,
    pub sender: Sender<Change>,
}

impl Platform {
    fn read_sample(&self, i: usize) -> Result<Vec<f32>, Box<dyn Error>> {
        let path = self.root.join(format!("samples/{}", DEFAULT_SAMPLES[i]));
        let config = DecoderConfig::new(Format::F32, 2, SAMPLE_RATE as u32);
        let mut decoder = Decoder::from_file(&path, Some(&config))?;
        let frame_count = decoder.length_in_pcm_frames() as usize;
        let mut samples = vec![0.0; 2 * frame_count];
        decoder.read_pcm_frames(&mut FramesMut::wrap(&mut samples[..], Format::F32, 2));
        Ok(samples)
    }
}

struct FindButton<'a>(&'static str, &'a mut ParamIndex);

impl<'a> UI<f32> for FindButton<'a> {
    fn add_button(&mut self, label: &'static str, i: ParamIndex) {
        if label == self.0 {
            *self.1 = i;
        }
    }
}


#[derive(Default)]
struct Song {
    note_index: ParamIndex,
    gate_index: ParamIndex,
    state: State<Song>,
    tracks: [Track; TRACK_COUNT],
    frames_since_last_step: AtomicCell<usize>,
}

impl Host for Song {
    fn for_each_param<F: FnMut(&'static str, &Param)>(f: &mut F) {
        effects::reverb::for_each_param(f);
        effects::echo::for_each_param(f);
        effects::drive::for_each_param(f);
        f("activeTrack", Param::new(0).max(TRACK_COUNT - 1).temp());
        f("playing", Param::new(false).temp());
        f("recording", Param::new(false).toggle().temp());
        f("root", Param::new(0).min(-12).max(12).step(7));
        f("scale", Param::new(0).max(SCALE_OFFSETS.len() - 1));
        f("step", Param::new(0).temp());
        f("tempo", Param::new(120).max(999).step(10));
    }
}

impl Song {
    fn new(platform: &Platform, json: &Value) -> Self {
        let mut song = Song::default();
        song.state.load(json);
        for (i, track) in song.tracks.iter_mut().enumerate() {
            let json = &json["tracks"][i];
            track.state.load(json);
            track.file_sample = platform.read_sample(i).expect("track.file_sample");
            if let Ok(live) = Vec::<[u8; 4]>::deserialize(&json["live"]) {
                track.live_length.store(live.len());
                for (atom, bytes) in track.live_sample.iter().zip(live.into_iter()) {
                    atom.store(f32::from_le_bytes(bytes));
                }
            }
            if let Ok(sequence) = Vec::<(usize, EventType, i32)>::deserialize(&json["sequence"]) {
                for (i, event_type, value) in sequence {
                    match event_type {
                        EventType::Key(key) => track.sequence[i].keys[key].set(value),
                    }
                }
            }
        }
        effects::insert::build_user_interface_static(&mut FindButton("note", &mut song.note_index));
        effects::insert::build_user_interface_static(&mut FindButton("gate", &mut song.gate_index));
        song.update_derived();
        song
    }

    fn active_track(&self) -> &Track {
        &self.tracks[self.state.get::<usize>("activeTrack")]
    }

    fn note(&self, track: &Track, key: usize) -> i32 {
        let note_id = key % SCALE_LENGTH;
        (track.state.get::<usize>("octave") + key / 7) as i32 * 12
            + if track.state.is("usingKey") {
                SCALE_OFFSETS[self.state.get::<usize>("scale")][note_id]
                    + self.state.get::<i32>("root")
            } else {
                SCALE_OFFSETS[0][note_id]
            }
    }

    fn quantized_step(&self, step: usize, resolution: usize) -> usize {
        let scale = MAX_RES / resolution;
        let scaled_step = step / scale * scale;
        let snap_to_next = (step - scaled_step) as f32 * self.step_duration(MAX_RES)
            + self.frames_since_last_step.load() as f32
            > self.step_duration(resolution) / 2.;
        scaled_step + scale * (snap_to_next as usize)
    }

    fn step_duration(&self, resolution: usize) -> f32 {
        SAMPLE_RATE as f32 * 240. / self.state.get::<f32>("tempo") / (resolution as f32)
    }

    fn update_derived(&self) {
        let track = self.active_track();
        track.state.set("canClear", track.can_clear());
        track.state.set("bars", track.bars());
        track.state.set("viewStart", track.view_start());
        for (i, name) in NOTE.iter().enumerate() {
            track.state.set(name, self.note(track, i));
        }
        for (i, name) in VIEW.iter().enumerate() {
            track.state.set(name, track.view(i));
        }
        for (i, name) in WAVEFORM.iter().enumerate() {
            track.state.set(name, track.waveform(i));
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
    key: usize,
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
                if track.sample_type().thru() {
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
                match track.sample_type() {
                    SampleType::File => self.play_sample(mix, &track.file_sample, 2),
                    SampleType::Live => self.play_thru(mix, track, input, false),
                    SampleType::LiveRecord => self.play_thru(mix, track, input, true),
                    SampleType::LivePlay => self.play_sample(mix, track.live(), 1),
                }
            }
        }
        buffer.compute(self.insert.dsp.as_mut());
        Buffer::write_over(&buffer.out, output);
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

    fn play_sample<T: CopyAs<f32>>(&mut self, mix: &mut [f32], sample: &[T], channels: usize) {
        let position = self.position.floor() as usize;
        let position_fract = self.position.fract();
        self.play(mix, |channel| {
            let index = |i| i * channels + channel % channels;
            if position_fract == 0. {
                sample.get(index(position)).map_or(0., T::copy_as)
            } else {
                let a = sample.get(index(position)).map_or(0., T::copy_as);
                let b = sample.get(index(position + 1)).map_or(0., T::copy_as);
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

impl Buffer<0, 0> {
    fn write_over(source: &[f32], destination: &mut [f32]) {
        for (destination, source) in destination.iter_mut().zip(source) {
            *destination += source;
        }
    }
}

struct SetParams<'a, T: ?Sized, S> {
    dsp: &'a mut T,
    state: &'a State<S>,
}

impl<'a, T: FaustDsp<T = f32> + ?Sized, S> UI<f32> for SetParams<'a, T, S> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.dsp.set_param(i, self.state.get(s));
    }
}

enum Task {
    WithI32(fn(&mut Audio, &Song, i32)),
    WithUsize(fn(&mut Audio, &Song, usize)),
    NudgeSong(&'static str),
    NudgeTrack(&'static str),
}

struct Audio {
    platform: Mutex<Platform>,
    voices: Vec<Voice>,
    sends: [DspDyn; SEND_COUNT],
    receiver: Arc<Mutex<Receiver<(Task, i32)>>>,
}

impl Audio {
    fn key_down(&mut self, song: &Song, track_id: usize, key: usize) {
        let track = &song.tracks[track_id];
        let song_step = song.state.get::<usize>("step");
        if song.state.is("playing") && song.state.is("recording") {
            let quantized_step = song.quantized_step(song_step, track.state.get("resolution"));
            let step = &track.sequence[quantized_step % track.state.get::<usize>("length")];
            let event = &step.keys[key];
            event.active.store(true);
            event.skip_next.store(quantized_step > song_step);
        }
        track.state.set("activeKey", key);
        self.allocate(song, track_id, key);
    }

    fn key_up(&mut self, song: &Song, track_id: usize, key: usize) {
        let track = &song.tracks[track_id];
        if song.state.is("playing") && song.state.is("recording") {
            let last_played = track.last_played[key].load();
            let quantized_step = song.quantized_step(last_played, track.state.get("resolution"));
            let step = &track.sequence[quantized_step % track.state.get::<usize>("length")];
            step.keys[key]
                .value
                .store((song.state.get::<usize>("step") - last_played) as i32);
        }
        self.release(track_id, key);
    }

    fn set_sample_type(&mut self, song: &Song, value: usize) {
        let track = song.active_track();
        let sample_type = SampleType::from(value);
        if sample_type == SampleType::LiveRecord {
            track.live_length.store(0);
        }
        if sample_type != track.sample_type() {
            song.active_track().state.set("sampleType", sample_type);
            for voice in self.each_voice_for(song.state.get("activeTrack")) {
                voice.gate = 0;
                voice.track_id = None;
            }
            if sample_type.thru() {
                self.allocate(
                    song,
                    song.state.get("activeTrack"),
                    track.state.get("activeKey"),
                );
            }
        }
    }

    fn toggle_play(&mut self, song: &Song) {
        song.state.toggle("playing");
        song.state.set("step", 0);
        song.frames_since_last_step.store(0);
        if !song.state.is("playing") {
            self.voices.iter_mut().for_each(|voice| voice.gate = 0);
        }
    }

    fn process(&mut self, song: &Song, input: &Frames, output: &mut FramesMut) {
        // Handle incoming audio tasks
        let receiver = Arc::clone(&self.receiver);
        let receiver = receiver.lock().expect("receiver");
        while let Ok((callback, data)) = receiver.try_recv() {
            match callback {
                Task::WithI32(f) => f(self, song, data),
                Task::WithUsize(f) => f(self, song, data as usize),
                Task::NudgeSong(name) => song.state.nudge(name, data),
                Task::NudgeTrack(name) => song.active_track().state.nudge(name, data),
            }
        }

        // Update DSP parameters
        for voice in self.voices.iter_mut() {
            if let Some(track_id) = voice.track_id {
                let dsp = voice.insert.dsp.as_mut();
                let state = &song.tracks[track_id].state;
                dsp.set_param(song.gate_index, voice.gate as f32);
                effects::insert::build_user_interface_static(&mut SetParams { dsp, state });
            }
        }
        for DspDyn { dsp, builder } in self.sends.iter_mut() {
            let dsp = dsp.as_mut();
            let state = &song.state;
            builder(&mut SetParams { dsp, state });
        }

        // Read input frames and calculate output frames
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            // If this is a new step, then replay any sequenced events
            if song.state.is("playing") && song.frames_since_last_step.load() == 0 {
                for (track_id, track) in song.tracks.iter().enumerate() {
                    let length = song.tracks[track_id].state.get::<usize>("length");
                    let song_step = song.state.get::<usize>("step");
                    let step = &track.sequence[song_step % length];
                    for (key, event) in step.keys.iter().enumerate() {
                        // Check if key should be released per its sequenced duration
                        let duration = song_step - track.last_played[key].load();
                        let start_step = &track.sequence[track.last_played[key].load() % length];
                        if duration as i32 == start_step.keys[key].value.load() {
                            self.release(track_id, key);
                        }

                        // Check if key should be played
                        if event.skip_next.load() {
                            event.skip_next.store(false);
                        } else if event.active.load() && !track.state.is("muted") {
                            self.allocate(song, track_id, key);
                        }
                    }
                }
            }

            // Advance song step
            if song.state.is("playing") {
                let next_step = song.frames_since_last_step.load() + 1;
                song.frames_since_last_step.store(next_step);
                if next_step as f32 >= song.step_duration(MAX_RES) {
                    song.frames_since_last_step.store(0);
                    song.state.add("step", 1);
                }
            }

            // Run voices and sends
            let mut buffer = Buffer::<INSERT_OUTPUT_COUNT, 2>::new();
            for voice in self.voices.iter_mut() {
                voice.process(song, input, &mut buffer.mix);
            }
            Buffer::write_over(&buffer.mix, output);
            for DspDyn { dsp, .. } in self.sends.iter_mut() {
                buffer.mix_start += 2;
                buffer.compute(dsp.as_mut());
                Buffer::write_over(&buffer.out, output);
            }
        }

        // Inform UI of changed state keys
        song.update_derived();
        let platform = self.platform.lock().expect("platform");
        let send = move |change| platform.sender.send(change).unwrap();
        song.state
            .for_each_change(|name, value| send(Change::Song(name, value)));
        for (i, track) in song.tracks.iter().enumerate() {
            track
                .state
                .for_each_change(|name, value| send(Change::Track(i, name, value)));
        }
    }

    fn allocate(&mut self, song: &Song, track_id: usize, key: usize) {
        self.release(track_id, key);
        let track = &song.tracks[track_id];
        let note = song.note(track, key) as f32;
        for voice in self.voices.iter_mut() {
            voice.age += 1;
        }

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
            voice.increment = ((note + track.state.get::<f32>("sampleDetune") / 10.) / 12.).exp2()
                / (69.0_f32 / 12.).exp2();
            voice.track_id = Some(track_id);
            voice.insert.dsp.instance_clear();
            voice.insert.dsp.set_param(song.note_index, note);
        }

        // Remember when this was played to for note length sequencer calculation
        track.last_played[key].store(song.state.get::<usize>("step"));

        // Inform UI
        track.state.add("recent", 1);
    }

    fn release(&mut self, track_id: usize, key: usize) {
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

#[derive(Default, Serialize)]
pub struct Save {
    song: HashMap<&'static str, Value>,
    tracks: [HashMap<&'static str, Value>; TRACK_COUNT],
}

#[derive(Clone)]
pub struct Controller {
    device: Device,
    song: Arc<RwLock<Song>>,
    audio: Arc<RwLock<Audio>>,
    sender: Arc<Mutex<Sender<(Task, i32)>>>,
}

impl Controller {
    pub fn stop(&self) {
        let _ = self.device.stop();
    }

    pub fn start(&self) {
        let _ = self.device.start();
    }

    pub fn load(&self, json: Value) {
        self.stop();
        {
            let mut song = self.song.write().expect("song");
            let audio = self.audio.read().expect("audio");
            let platform = audio.platform.lock().expect("platform");
            *song = Song::new(&platform, &json);
        }
        self.start();
    }

    pub fn save(&self, strategy: Strategy) -> impl Serialize {
        let song = self.song.read().expect("song");
        let mut save = Save::default();
        song.state.save(strategy, &mut save.song);
        for (song_track, save_track) in song.tracks.iter().zip(save.tracks.iter_mut()) {
            if matches!(strategy, Strategy::File) {
                save_track.insert(
                    "live",
                    song_track
                        .live()
                        .iter()
                        .map(|atom| Vec::from(atom.load().to_le_bytes()))
                        .collect(),
                );
                save_track.insert(
                    "sequence",
                    song_track
                        .events()
                        .enumerate()
                        .filter(|(_, (_, event))| event.active.load())
                        .map(|(i, (event_type, event))| {
                            serde_json::to_value((i, event_type, &event.value)).expect("event")
                        })
                        .collect(),
                );
            }
            song_track.state.save(strategy, save_track);
        }
        save
    }

    pub fn send(&self, method: &str, data: i32) {
        let callback = match method {
            "activeTrack" => Task::WithUsize(|audio, song, i| {
                song.state.set("activeTrack", i);
                if !song.state.is("playing") {
                    let id = song.state.get("activeTrack");
                    audio.key_down(song, id, song.tracks[id].state.get("activeKey"));
                }
            }),
            "auditionDown" => Task::WithUsize(|audio, song, i| {
                audio.key_down(song, i, song.tracks[i].state.get("activeKey"))
            }),
            "auditionUp" => Task::WithUsize(|audio, song, i| {
                audio.key_up(song, i, song.tracks[i].state.get("activeKey"))
            }),
            "length" => Task::WithI32(|_, song, i| song.active_track().adjust_length(i)),
            "clear" => Task::WithUsize(|_, song, _| song.active_track().clear()),
            "muted" => Task::WithUsize(|_, song, i| song.tracks[i].state.toggle("muted")),
            "noteDown" => Task::WithUsize(|audio, song, i| {
                audio.key_down(song, song.state.get("activeTrack"), i)
            }),
            "noteUp" => Task::WithUsize(|audio, song, i| {
                audio.key_up(song, song.state.get("activeTrack"), i)
            }),
            "page" => Task::WithI32(|_, song, i| song.active_track().adjust_page(i)),
            "playing" => Task::WithUsize(|audio, song, _| audio.toggle_play(song)),
            "sampleType" => Task::WithUsize(|audio, song, i| audio.set_sample_type(song, i)),
            "sequence" => Task::WithUsize(|_, song, i| song.active_track().toggle_step(i)),
            "taps" => Task::WithUsize(|_, song, i| song.state.set("tempo", i)),
            "zoomIn" => Task::WithUsize(|_, song, _| song.active_track().zoom_in()),
            "zoomOut" => Task::WithUsize(|_, song, _| song.active_track().zoom_out()),
            _ => {
                let song = self.song.read().expect("song");
                if let Some(name) = song.state.find(method) {
                    Task::NudgeSong(name)
                } else if let Some(name) = song.active_track().state.find(method) {
                    Task::NudgeTrack(name)
                } else {
                    return;
                }
            }
        };
        let _ = self.sender.lock().expect("sender").send((callback, data));
    }
}

pub fn init(platform: Platform, json: Value) -> Result<Controller, Box<dyn Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let voice_count = platform.voice_count;
    let audio = Audio {
        platform: Mutex::new(platform),
        voices: vec![Voice::default(); voice_count],
        sends: [
            ("reverb", DspBox::<effects::reverb>::default()).into(),
            ("echo", DspBox::<effects::echo>::default()).into(),
            ("drive", DspBox::<effects::drive>::default()).into(),
        ],
        receiver: Arc::new(Mutex::new(receiver)),
    };
    for voice in audio.voices.iter() {
        assert_eq!(voice.insert.dsp.get_num_inputs(), 2);
        assert_eq!(
            voice.insert.dsp.get_num_outputs(),
            INSERT_OUTPUT_COUNT as i32
        );
    }
    for DspDyn { dsp, .. } in audio.sends.iter() {
        assert_eq!(dsp.get_num_inputs(), 2);
        assert_eq!(dsp.get_num_outputs(), 2);
    }

    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(SAMPLE_RATE as u32);

    let song = Arc::new(RwLock::new(Song::new(
        &audio.platform.lock().expect("platform"),
        &json,
    )));
    let audio = Arc::new(RwLock::new(audio));
    let controller_song = Arc::clone(&song);
    let controller_audio = Arc::clone(&audio);
    let mut device = Device::new(None, &device_config)?;
    device.set_data_callback(move |_, output, input| {
        audio
            .try_write()
            .expect("audio")
            .process(&song.try_read().expect("song"), input, output);
    });

    Ok(Controller {
        device,
        song: controller_song,
        audio: controller_audio,
        sender: Arc::new(Mutex::new(sender)),
    })
}
