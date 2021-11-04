#![feature(array_methods)]
#![feature(bool_to_option)]
#![feature(format_args_capture)]

use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};

use default_boxed::DefaultBoxed;
use miniaudio::{
    Decoder, DecoderConfig, Device, DeviceConfig, DeviceType, Format, Frames, FramesMut,
};
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use atomic_cell::{AtomicCell, CopyAs};
use effects::{FaustDsp, ParamIndex, UI};
pub use state::Strategy;
use state::{Bind, Host, Param, State, Visitor};

mod atomic_cell;
mod effects;
mod state;

const SEND_COUNT: usize = 3;
const INSERT_OUTPUT_COUNT: usize = 2 + 2 * SEND_COUNT;

const MAX_RESOLUTION: usize = 512;
const MAX_LENGTH: usize = MAX_RESOLUTION * 8;
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
    "KR-33 Kick1.wav",
    "KR-33 Kick2.wav",
    "KR-33 Tom-Lo.wav",
    "KR-33 Tom-Mid.wav",
    "KR-33 Tom-Hi.wav",
    "KR-33 Snare1.wav",
    "KR-33 Snare2.wav",
    "KR-33 Hat-Cl1.wav",
    "KR-33 Hat-Cl2.wav",
    "KR-33 Hat-Op.wav",
    "KR-33 Cow.wav",
    "KR-33 Crash.wav",
    "KR-33 Cymbal.wav",
    "KR-33 Rim.wav",
    "Tag.wav",
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

#[derive(PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(usize)]
pub enum SampleType {
    File,
    Live,
    LiveRecord,
    LivePlay,
}

impl Param for SampleType {}

impl SampleType {
    fn thru(self) -> bool {
        match self {
            Self::File | Self::LivePlay => false,
            Self::Live | Self::LiveRecord => true,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Event {
    value: AtomicCell<i32>,
    active: AtomicCell<bool>,
    #[serde(skip)]
    skip_next: AtomicCell<bool>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Step {
    #[serde(default)]
    keys: [Event; KEY_COUNT as usize],
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(usize)]
pub enum View {
    OutOfBounds,
    Empty,
    ExactlyOnStep,
    ContainsSteps,
}

impl Param for View {}

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
    fn host<V: Visitor>(visitor: &mut V) {
        effects::insert::host(visitor);
        visitor.visit("activeKey", 12, &[Bind::Max(TRACK_COUNT - 1)]);
        visitor.visit("bars", 1, &[Bind::Temp]);
        visitor.visit("canClear", false, &[Bind::Temp]);
        visitor.visit(
            "length",
            MAX_RESOLUTION,
            &[
                Bind::Min(MAX_RESOLUTION as i32),
                Bind::Max(MAX_RESOLUTION * 8),
            ],
        );
        visitor.visit("muted", false, &[Bind::Toggle]);
        visitor.visit("octave", 4, &[Bind::Min(2), Bind::Max(8)]);
        visitor.visit("pageStart", 0, &[Bind::Temp]);
        visitor.visit("recent", 0, &[Bind::Temp]);
        visitor.visit("resolution", 16, &[Bind::Min(1), Bind::Max(MAX_RESOLUTION)]);
        visitor.visit("useKey", true, &[Bind::Toggle]);
        visitor.visit("viewStart", 0, &[Bind::Temp]);
        for name in NOTE.iter().chain(VIEW.iter()).chain(WAVEFORM.iter()) {
            visitor.visit(name, 0, &[Bind::Temp]);
        }
    }
}

impl Track {
    fn sample_type(&self) -> SampleType {
        self.state.get::<SampleType>("sampleType")
    }

    fn bars(&self) -> usize {
        self.state.get::<usize>("length") / MAX_RESOLUTION
    }

    fn view_start(&self) -> usize {
        self.state.get::<usize>("pageStart") / self.view_length()
    }

    fn view_length(&self) -> usize {
        MAX_RESOLUTION / self.state.get::<usize>("resolution")
    }

    fn view_from(&self, start: usize) -> View {
        if start >= self.state.get("length") {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in start..(start + self.view_length()) {
            let step = &self.sequence[i];
            let change = &step.keys[self.state.get::<usize>("activeKey")];
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
        if self.state.get::<usize>("resolution") < MAX_RESOLUTION {
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
            Track::adjust(self.state.get("length"), diff, MAX_RESOLUTION),
        );
    }

    fn toggle_step(&self, i: usize) {
        let start = self.view_index_to_start(i);
        match self.view_from(start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let change =
                    &self.sequence[start as usize].keys[self.state.get::<usize>("activeKey")];
                change.active.toggle();
                change.skip_next.store(false);
                change.value.store(self.view_length() as i32);
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
        self.events().any(|change| change.active.load())
    }

    fn clear(&self) {
        self.events().for_each(|change| change.active.store(false));
    }

    fn events(&self) -> impl Iterator<Item = &Event> {
        self.sequence
            .iter()
            .take(self.state.get("length"))
            .flat_map(|step| step.keys.iter())
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

#[derive(Default)]
struct Song {
    note_index: ParamIndex,
    gate_index: ParamIndex,
    state: State<Song>,
    tracks: [Track; TRACK_COUNT],
    step: AtomicCell<usize>,
    frames_since_last_step: AtomicCell<usize>,
}

impl Host for Song {
    fn host<V: Visitor>(visitor: &mut V) {
        effects::reverb::host(visitor);
        effects::echo::host(visitor);
        effects::drive::host(visitor);
        visitor.visit("activeTrack", 0, &[Bind::Max(TRACK_COUNT - 1), Bind::Temp]);
        visitor.visit("playing", false, &[Bind::Temp]);
        visitor.visit("recording", false, &[Bind::Temp, Bind::Toggle]);
        visitor.visit("root", 0, &[Bind::Min(-12), Bind::Max(12), Bind::Step(7)]);
        visitor.visit("scale", 0, &[Bind::Max(SCALE_OFFSETS.len() - 1)]);
        visitor.visit("tempo", 120, &[Bind::Max(999), Bind::Step(10)]);
    }
}

impl Song {
    fn init(&mut self, platform: &Platform) {
        for (i, track) in self.tracks.iter_mut().enumerate() {
            track.file_sample = platform.read_sample(i).expect("track.file_sample");
        }
        self.update_derived();
    }

    fn active_track(&self) -> &Track {
        &self.tracks[self.state.get::<usize>("activeTrack")]
    }

    fn note(&self, track: &Track, key: usize) -> i32 {
        let note_id = key % SCALE_LENGTH;
        (track.state.get::<usize>("octave") + key / 7) as i32 * 12
            + if track.state.get("useKey") {
                SCALE_OFFSETS[self.state.get::<usize>("scale")][note_id]
                    + self.state.get::<i32>("root")
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
        let song_step = song.step.load();
        if song.state.get::<bool>("playing") && song.state.get("recording") {
            let quantized_step = song.quantized_step(song_step, track.state.get("resolution"));
            let step = &track.sequence[quantized_step % track.state.get::<usize>("length")];
            let change = &step.keys[key];
            change.active.store(true);
            change.skip_next.store(quantized_step > song_step);
        }
        track.state.set("activeKey", key);
        self.allocate(song, track_id, key);
    }

    fn key_up(&mut self, song: &Song, track_id: usize, key: usize) {
        let track = &song.tracks[track_id];
        if song.state.get::<bool>("playing") && song.state.get("recording") {
            let last_played = track.last_played[key].load();
            let quantized_step = song.quantized_step(last_played, track.state.get("resolution"));
            let step = &track.sequence[quantized_step % track.state.get::<usize>("length")];
            step.keys[key]
                .value
                .store((song.step.load() - last_played) as i32);
        }
        self.release(track_id, key);
    }

    fn set_sample_type(&mut self, song: &Song, value: usize) {
        let track = song.active_track();
        let old = track.sample_type();
        let new = SampleType::transcode(value);
        if new == SampleType::LiveRecord {
            track.live_length.store(0);
        }
        if old != new {
            song.active_track().state.set("sampleType", value as i32);
            for voice in self.each_voice_for(song.state.get::<usize>("activeTrack")) {
                voice.gate = 0;
                voice.track_id = None;
            }
            if new.thru() {
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
        song.step.store(0);
        song.frames_since_last_step.store(0);
        if !song.state.get::<bool>("playing") {
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
            if song.state.get::<bool>("playing") && song.frames_since_last_step.load() == 0 {
                for (track_id, track) in song.tracks.iter().enumerate() {
                    let length = song.tracks[track_id].state.get::<usize>("length");
                    let song_step = song.step.load();
                    let step = &track.sequence[song_step % length];
                    for (key, change) in step.keys.iter().enumerate() {
                        // Check if key should be released per its sequenced duration
                        let duration = song_step - track.last_played[key].load();
                        let start_step = &track.sequence[track.last_played[key].load() % length];
                        if duration as i32 == start_step.keys[key].value.load() {
                            self.release(track_id, key);
                        }

                        // Check if key should be played
                        if change.skip_next.load() {
                            change.skip_next.store(false);
                        } else if change.active.load() && !track.state.get::<bool>("muted") {
                            self.allocate(song, track_id, key);
                        }
                    }
                }
            }

            // Advance song step
            if song.state.get::<bool>("playing") {
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
        track.last_played[key].store(song.step.load());

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

    pub fn dump(&self) -> impl Serialize {
        #[derive(Serialize)]
        pub struct Dump<S, T> {
            song: S,
            tracks: Vec<T>,
        }
        let song = self.song.read().expect("song");
        Dump {
            song: song.state.save(Strategy::Dump),
            tracks: song
                .tracks
                .iter()
                .map(|track| track.state.save(Strategy::Dump))
                .collect(),
        }
    }

    pub fn send(&self, method: &str, data: i32) {
        let callback = match method {
            "activeTrack" => Task::WithUsize(|audio, song, i| {
                song.state.set("activeTrack", i);
                if !song.state.get::<bool>("playing") {
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

pub fn init(platform: Platform) -> Result<Controller, Box<dyn Error>> {
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

    let mut song = Song::default();
    song.init(&audio.platform.lock().expect("platform"));

    struct FindButton<'a>(&'static str, &'a mut ParamIndex);
    impl<'a> UI<f32> for FindButton<'a> {
        fn add_button(&mut self, label: &'static str, i: ParamIndex) {
            if label == self.0 {
                *self.1 = i;
            }
        }
    }
    effects::insert::build_user_interface_static(&mut FindButton("note", &mut song.note_index));
    effects::insert::build_user_interface_static(&mut FindButton("gate", &mut song.gate_index));

    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(SAMPLE_RATE as u32);

    let song = Arc::new(RwLock::new(song));
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
