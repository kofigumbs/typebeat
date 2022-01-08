#![feature(array_methods)]
#![feature(bool_to_option)]
#![feature(format_args_capture)]

use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};

use miniaudio::{
    Decoder, DecoderConfig, Device, DeviceConfig, DeviceType, Format, Frames, FramesMut,
};
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub use atomic_cell::{AtomicCell, CopyAs};
use effects::{FaustDsp, ParamIndex, UI};
use state::{Host, Param, State};

mod atomic_cell;
mod effects;
mod state;

const INSERT_INPUT_COUNT: usize = 4;
const INSERT_OUTPUT_COUNT: usize = 8;

const KEY_COUNT: usize = 15;
const SAMPLE_RATE: usize = 44100;
const TRACK_COUNT: usize = 15;
const VIEWS_PER_PAGE: usize = 4;

const MAX_RES: usize = 512;
const MAX_LENGTH: usize = MAX_RES * 8;

const SCALE_LENGTH: usize = 7;
const SCALE_OFFSETS: &[[i32; SCALE_LENGTH]] = &[
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
    [0, 2, 3, 5, 7, 9, 11],
];

const DEFAULT_SAMPLES: &[&str] = &[
    "kick", "kickme", "tom", "conga", "cabasa", "sd", "sst", "clap", "cowb", "tamb", "chhl",
    "chhs", "crash", "ride", "tag",
];

lazy_static::lazy_static! {
    static ref NOTE: Vec<String> = (0..TRACK_COUNT).map(|i| format!("note{i}")).collect();
    static ref ROLL: Vec<String> = (0..VIEWS_PER_PAGE*KEY_COUNT).map(|i| format!("roll{i}")).collect();
    static ref VIEW: Vec<String> = (0..VIEWS_PER_PAGE).map(|i| format!("view{i}")).collect();
    static ref VIEW_INDEX: Vec<String> = (0..VIEWS_PER_PAGE).map(|i| format!("viewIndex{i}")).collect();
    static ref WAVEFORM: Vec<String> = (0..25).map(|i| format!("waveform{i}")).collect();
}

fn host_each<P, F: FnMut(&'static str, &P)>(f: &mut F, names: &'static [String], param: &P) {
    names.iter().for_each(|name| f(name, param));
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
        dsp.init(SAMPLE_RATE as i32);
        DspBox { dsp }
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(usize)]
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
    /// Does this sample stream mic input?
    fn through(self) -> bool {
        match self {
            Self::File | Self::LivePlay => false,
            Self::Live | Self::LiveRecord => true,
        }
    }
}

/// Unit for note sequencing
#[derive(Clone, Default)]
struct Hit {
    active: AtomicCell<bool>,
    /// Sometimes quantizing can activate a future step -- this flag prevents double triggers
    skip_next: AtomicCell<bool>,
    /// Hold time in 512th notes
    duration: AtomicCell<usize>,
}

/// Wrapper for Hit that's easier to save/load
#[derive(Deserialize, Serialize)]
struct HitId<T> {
    step: usize,
    key: usize,
    hit: T,
}

impl<'a> HitId<&'a Hit> {
    fn save(HitId { step, key, hit }: Self) -> Option<Value> {
        if !hit.active.load() {
            None
        } else {
            let hit = hit.duration.load();
            serde_json::to_value(HitId { step, key, hit }).ok()
        }
    }
}

/// Summary of a subsequence of Hits
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
    sequence: Vec<[Hit; KEY_COUNT]>,
    /// Tracks when keys were last triggered for duration/release calculations
    last_played: [AtomicCell<usize>; KEY_COUNT],
}

impl Default for Track {
    fn default() -> Self {
        let mut live_sample = Vec::new();
        live_sample.resize_with(60 * SAMPLE_RATE, Default::default);
        Self {
            state: State::default(),
            file_sample: Vec::new(),
            live_sample,
            live_length: 0.into(),
            sequence: vec![Default::default(); MAX_LENGTH as usize],
            last_played: Default::default(),
        }
    }
}

impl Host for Track {
    fn host<F: FnMut(&'static str, &Param)>(f: &mut F) {
        effects::insert::host(f);
        f("activeKey", Param::new(12).min(0).max(TRACK_COUNT - 1));
        f("bars", Param::new(1).temp());
        f("canClear", Param::new(false).temp());
        f("length", Param::new(MAX_RES).min(MAX_RES).max(MAX_RES * 8));
        f("muted", Param::new(false).toggle());
        f("octave", Param::new(4).min(2).max(8).step(2));
        f("pageStart", Param::new(0).min(0).temp());
        f("recent", Param::new(0).temp());
        f("resolution", Param::new(16).min(1).max(MAX_RES));
        f("usingKey", Param::new(false).toggle());
        f("viewLength", Param::new(0).temp());
        f("viewStart", Param::new(0).temp());
        host_each(f, &NOTE, Param::new(0).temp());
        host_each(f, &ROLL, Param::new(0).temp());
        host_each(f, &VIEW, Param::new(0).temp());
        host_each(f, &VIEW_INDEX, Param::new(0).temp());
        host_each(f, &WAVEFORM, Param::new(0).temp());
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

    fn view_from(&self, key: usize, start: usize) -> View {
        if start >= self.state.get("length") {
            return View::OutOfBounds;
        }
        let mut active_count = 0;
        let mut last_active = 0;
        for i in (start..).take(self.view_length()) {
            let hit = &self.sequence[i][key];
            if hit.active.load() {
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

    fn view_index(&self, i: usize) -> usize {
        self.state.get::<usize>("pageStart") + i * self.view_length()
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
        let new_page_start = self
            .state
            .get::<i32>("pageStart")
            .saturating_add(diff * (VIEWS_PER_PAGE * self.view_length()) as i32);
        if new_page_start < self.state.get("length") {
            self.state.set("pageStart", new_page_start);
        }
    }

    fn adjust_length(&self, diff: i32) {
        self.state.add("length", diff * MAX_RES as i32);
    }

    fn toggle_step(&self, i: usize) {
        let start = self.view_index(i);
        let key = self.state.get("activeKey");
        match self.view_from(key, start) {
            View::OutOfBounds => {}
            View::Empty | View::ExactlyOnStep => {
                let hit = &self.sequence[start as usize][key];
                hit.active.toggle();
                hit.skip_next.store(false);
                hit.duration.store(self.view_length());
            }
            View::ContainsSteps => {
                for i in start..(start + self.view_length()) {
                    self.sequence[i][key].active.store(false);
                }
            }
        }
    }

    fn can_clear(&self) -> bool {
        self.hits().any(|hit_id| hit_id.hit.active.load())
    }

    fn clear(&self) {
        self.hits()
            .for_each(|hit_id| hit_id.hit.active.store(false));
    }

    fn hits(&self) -> impl Iterator<Item = HitId<&Hit>> {
        self.sequence
            .iter()
            .take(self.state.get("length"))
            .enumerate()
            .flat_map(|(step, keys)| {
                keys.iter()
                    .enumerate()
                    .map(move |(key, hit)| HitId { step, key, hit })
            })
    }

    fn live(&self) -> &[AtomicCell<f32>] {
        &self.live_sample[..self.live_length.load()]
    }

    fn waveform(&self, i: usize) -> f32 {
        match self.sample_type() {
            SampleType::File => self.sample_waveform(i, &self.file_sample),
            SampleType::LivePlay => self.sample_waveform(i, self.live()),
            SampleType::Live | SampleType::LiveRecord => 0.,
        }
    }

    fn sample_waveform<T: CopyAs<f32>>(&self, i: usize, sample: &[T]) -> f32 {
        100. * sample
            .chunks(sample.len() / WAVEFORM.len())
            .nth(i)
            .unwrap_or_default()
            .into_iter()
            .fold(0., |max, f| f.copy_as().abs().max(max))
    }
}

/// State sync events (for JavaScript)
pub enum Change {
    Dump(Value),
    Song(&'static str, i32),
    Track(usize, &'static str, i32),
}

impl Serialize for Change {
    /// JSON can always be passed directly to store setter (https://www.solidjs.com/docs/latest/api#createstore)
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Change::Dump(value) => (value,).serialize(serializer),
            Change::Song(method, value) => ("song", method, value).serialize(serializer),
            Change::Track(i, method, value) => ("tracks", i, method, value).serialize(serializer),
        }
    }
}

/// Client-specific configuration
pub struct Platform {
    pub voice_count: usize,
    pub root: PathBuf,
    pub sender: Sender<Change>,
}

impl Platform {
    fn read_sample(&self, i: usize) -> Result<Vec<f32>, Box<dyn Error>> {
        let path = self
            .root
            .join(format!("samples/{}.wav", DEFAULT_SAMPLES[i]));
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
    duck_release_index: ParamIndex,
    state: State<Song>,
    tracks: [Track; TRACK_COUNT],
    frames_since_last_step: AtomicCell<usize>,
}

impl Host for Song {
    fn host<F: FnMut(&'static str, &Param)>(f: &mut F) {
        effects::echo::host(f);
        effects::reverb::host(f);
        f(
            "activeTrack",
            Param::new(0).min(0).max(TRACK_COUNT - 1).temp(),
        );
        f("duckRelease", Param::new(25).min(0).max(50).step(10));
        f("playing", Param::new(false).temp());
        f("recording", Param::new(false).toggle().temp());
        f("root", Param::new(0).min(-12).max(12).step(7));
        f("scale", Param::new(0).max(SCALE_OFFSETS.len() - 1));
        f("step", Param::new(0).temp());
        f("tempo", Param::new(120).min(0).max(999).step(10));
    }
}

impl Song {
    fn new(platform: &Platform, json: &Value) -> Self {
        let mut song = Song::default();
        song.state.init(&json["song"]);
        for (i, track) in song.tracks.iter_mut().enumerate() {
            let json = &json["tracks"][i];
            track.state.init(json);
            track.file_sample = platform.read_sample(i).unwrap();
            <&str>::deserialize(&json["live"]).ok().map(|s| {
                let live = AtomicCell::from_base64(s);
                track.live_length.store(live.len());
                for (frame, x) in track.live_sample.iter_mut().zip(live.into_iter()) {
                    *frame = x;
                }
            });
            Vec::<HitId<usize>>::deserialize(&json["sequence"])
                .unwrap_or_default()
                .into_iter()
                .filter(|hit_id| hit_id.step < track.sequence.len() && hit_id.key < KEY_COUNT)
                .for_each(|hit_id| {
                    let hit = &track.sequence[hit_id.step][hit_id.key];
                    hit.active.store(true);
                    hit.duration.store(hit_id.hit);
                });
        }

        // Initialize the note, gate, and duckRelease Faust ids
        Buttons::find::<effects::insert>("note", &mut song.note_index);
        Buttons::find::<effects::insert>("gate", &mut song.gate_index);
        Buttons::find::<effects::insert>("duckRelease", &mut song.duck_release_index);

        song.update_derived();
        song
    }

    fn save(&self) -> impl Serialize {
        serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "song": self.state.save(),
            "tracks": self.tracks.iter().map(|track| {
                let mut map = track.state.save();
                map.insert("live", AtomicCell::to_base64(track.live()).into());
                map.insert("sequence", track.hits().filter_map(HitId::save).collect());
                map
            })
            .collect::<Vec<_>>(),
        })
    }

    fn dump(&self) -> impl Serialize {
        serde_json::json!({
            "song": self.state.dump(),
            "tracks": self.tracks.iter().map(|track| track.state.dump()).collect::<Vec<_>>(),
        })
    }

    fn active_track(&self) -> &Track {
        &self.tracks[self.state.get::<usize>("activeTrack")]
    }

    fn note(&self, track: &Track, key: usize) -> i32 {
        let id = key % SCALE_LENGTH;
        let base = (track.state.get::<usize>("octave") + key / 7) as i32 * 12;
        let offset = if track.state.is("usingKey") {
            SCALE_OFFSETS[self.state.get::<usize>("scale")][id] + self.state.get::<i32>("root")
        } else {
            SCALE_OFFSETS[0][id]
        };
        base + offset
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
        track.state.set("bars", track.bars());
        track.state.set("canClear", track.can_clear());
        track.state.set("viewStart", track.view_start());
        track.state.set("viewLength", track.view_length());
        for (i, name) in NOTE.iter().enumerate() {
            track.state.set(name, self.note(track, i));
        }
        for (i, name) in ROLL.iter().enumerate() {
            let key = i / VIEWS_PER_PAGE;
            let length = track.state.get::<usize>("length");
            let start = (self.state.get::<usize>("step") % length) / track.view_length()
                * track.view_length()
                + (i % VIEWS_PER_PAGE * track.view_length());
            track.state.set(name, track.view_from(key, start % length));
        }
        for (i, (view, view_index)) in VIEW.iter().zip(VIEW_INDEX.iter()).enumerate() {
            let key = track.state.get("activeKey");
            let start = track.view_index(i);
            track.state.set(view, track.view_from(key, start));
            track.state.set(view_index, start);
        }
        for (i, name) in WAVEFORM.iter().enumerate() {
            track.state.set(name, track.waveform(i));
        }
    }
}

/// Audio buffer for working with Faust arrays and slices
struct Buffer<const N: usize, const M: usize> {
    mix: [f32; N],
    out: [f32; M],
}

impl<const N: usize, const M: usize> Buffer<N, M> {
    fn new() -> Self {
        Buffer {
            mix: [0.; N],
            out: [0.; M],
        }
    }

    fn compute<T>(&mut self, dsp: &mut T, output: &mut [f32], mix_start: usize)
    where
        T: effects::FaustDsp<T = f32>,
    {
        dsp.compute(
            1,
            &self.mix.each_ref().map(std::slice::from_ref)[mix_start..],
            &mut self.out.each_mut().map(std::slice::from_mut),
        );
        for (destination, source) in output.iter_mut().zip(&self.out) {
            *destination += source;
        }
    }
}

/// Synchronizes params between dsp instance and state
struct Params<'a, T: ?Sized, S> {
    dsp: &'a mut T,
    state: &'a State<S>,
}

impl<'a, T: FaustDsp<T = f32>, S> UI<f32> for Params<'a, T, S> {
    fn add_num_entry(&mut self, s: &'static str, i: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.dsp.set_param(i, self.state.get(s));
    }
}

impl<'a, T: FaustDsp<T = f32>, S> Params<'a, T, S> {
    fn set(dsp: &'a mut T, state: &'a State<S>) {
        T::build_user_interface_static(&mut Self { dsp, state });
    }
}

/// Collects Faust button param indexes
struct Buttons<'a> {
    label: &'static str,
    index: &'a mut ParamIndex,
}

impl<'a> UI<f32> for Buttons<'a> {
    fn add_button(&mut self, label: &'static str, i: ParamIndex) {
        if label == self.label {
            *self.index = i;
        }
    }
}

impl<'a> Buttons<'a> {
    fn find<T: FaustDsp<T = f32>>(label: &'static str, index: &'a mut ParamIndex) {
        T::build_user_interface_static(&mut Buttons { label, index });
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
    /// Score voice to determine stealing order, 0 means "do not steal"
    fn priority(&self, song: &Song) -> usize {
        match self.track_id {
            Some(track_id) => {
                let track = &song.tracks[track_id];
                if track.sample_type().through() {
                    0
                } else {
                    2 - self.gate
                }
            }
            None => 4,
        }
    }

    fn process(&mut self, song: &Song, duck_mix: &[f32], input: &[f32], output: &mut [f32]) {
        let playback = match self.track_id {
            None => [0., 0.],
            Some(track_id) => {
                let track = &song.tracks[track_id];
                match track.sample_type() {
                    SampleType::File => self.play_sample(&track.file_sample, 2),
                    SampleType::Live => self.play_through(track, input, false),
                    SampleType::LiveRecord => self.play_through(track, input, true),
                    SampleType::LivePlay => self.play_sample(track.live(), 1),
                }
            }
        };
        self.position += self.increment;
        let mut buffer = Buffer::<INSERT_INPUT_COUNT, INSERT_OUTPUT_COUNT>::new();
        buffer.mix = [duck_mix[0], duck_mix[1], playback[0], playback[1]];
        buffer.compute(self.insert.dsp.as_mut(), output, 0);
    }

    fn play_through(&mut self, track: &Track, input: &[f32], record: bool) -> [f32; 2] {
        let input = input.iter().sum();
        let length = track.live_length.load();
        if record && length < track.live_sample.len() {
            track.live_sample[length].store(input);
            track.live_length.store(length + 1);
        }
        [input, input]
    }

    fn play_sample<T: CopyAs<f32>>(&mut self, sample: &[T], channels: usize) -> [f32; 2] {
        let position = self.position.floor() as usize;
        let position_fract = self.position.fract();
        [0, 1].map(|channel| {
            let index = |i| i * channels + channel % channels;
            if position_fract == 0. {
                sample.get(index(position)).map_or(0., T::copy_as)
            } else {
                let a = sample.get(index(position)).map_or(0., T::copy_as);
                let b = sample.get(index(position + 1)).map_or(0., T::copy_as);
                position_fract.mul_add(b - a, a)
            }
        })
    }

    fn set_params(&mut self, song: &Song, track_id: usize) {
        self.track_id = Some(track_id);
        self.insert.dsp.set_param(song.gate_index, self.gate as f32);
        self.insert
            .dsp
            .set_param(song.duck_release_index, song.state.get("duckRelease"));
        Params::set(self.insert.dsp.as_mut(), &song.tracks[track_id].state);
    }
}

/// User events (from JavaScript)
enum Command {
    WithI32(fn(&mut Audio, &Song, i32)),
    WithUsize(fn(&mut Audio, &Song, usize)),
    NudgeSong(&'static str),
    NudgeTrack(&'static str),
}

struct Audio {
    platform: Mutex<Platform>,
    voices: Vec<Voice>,
    duck_mix: [f32; 2],
    echo: DspBox<effects::echo>,
    reverb: DspBox<effects::reverb>,
    receiver: Arc<Mutex<Receiver<(Command, i32)>>>,
}

impl Audio {
    fn key_down(&mut self, song: &Song, track_id: usize, key: usize) {
        let track = &song.tracks[track_id];
        let song_step = song.state.get::<usize>("step");
        if song.state.is("playing") && song.state.is("recording") {
            let quantized_step = song.quantized_step(song_step, track.state.get("resolution"));
            let hit = &track.sequence[quantized_step % track.state.get::<usize>("length")][key];
            hit.active.store(true);
            hit.skip_next.store(quantized_step > song_step);
        }
        track.state.set("activeKey", key);
        self.allocate(song, track_id, key);
    }

    fn key_up(&mut self, song: &Song, track_id: usize, key: usize) {
        let track = &song.tracks[track_id];
        if song.state.is("playing") && song.state.is("recording") {
            let last_played = track.last_played[key].load();
            let quantized_step = song.quantized_step(last_played, track.state.get("resolution"));
            track.sequence[quantized_step % track.state.get::<usize>("length")][key]
                .duration
                .store(song.state.get::<usize>("step") - last_played);
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
            if sample_type.through() {
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
        song.state.set("recording", false);
        song.state.set("step", 0);
        song.frames_since_last_step.store(0);
        if !song.state.is("playing") {
            self.voices.iter_mut().for_each(|voice| voice.gate = 0);
        }
    }

    fn process(&mut self, song: &Song, input: &Frames, output: &mut FramesMut) {
        // Handle incoming audio commands
        let receiver = Arc::clone(&self.receiver);
        let receiver = receiver.lock().unwrap();
        while let Ok((callback, data)) = receiver.try_recv() {
            match callback {
                Command::WithI32(f) => f(self, song, data),
                Command::WithUsize(f) => f(self, song, data as usize),
                Command::NudgeSong(name) => song.state.nudge(name, data),
                Command::NudgeTrack(name) => song.active_track().state.nudge(name, data),
            }
        }

        // Update DSP parameters
        for voice in self.voices.iter_mut() {
            if let Some(track_id) = voice.track_id {
                voice.set_params(song, track_id);
            }
        }
        Params::set(self.echo.dsp.as_mut(), &song.state);
        Params::set(self.reverb.dsp.as_mut(), &song.state);

        // Read input frames and calculate output frames
        for (input, output) in input.frames::<f32>().zip(output.frames_mut::<f32>()) {
            // If this is a new step, then replay any sequenced hits
            if song.state.is("playing") && song.frames_since_last_step.load() == 0 {
                for (track_id, track) in song.tracks.iter().enumerate() {
                    let length = song.tracks[track_id].state.get::<usize>("length");
                    let song_step = song.state.get::<usize>("step");
                    for (key, hit) in track.sequence[song_step % length].iter().enumerate() {
                        // Check if last played hit should be released per its duration
                        let last_hit = &track.sequence[track.last_played[key].load() % length][key];
                        let duration = song_step - track.last_played[key].load();
                        if duration == last_hit.duration.load() {
                            self.release(track_id, key);
                        }

                        // Check if key should be played
                        if hit.skip_next.load() {
                            hit.skip_next.store(false);
                        } else if hit.active.load() && !track.state.is("muted") {
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
                voice.process(song, &self.duck_mix, input, &mut buffer.mix);
            }
            self.duck_mix.copy_from_slice(&buffer.mix[..2]); // duck
            output.copy_from_slice(&buffer.mix[2..4]); // main/dry
            buffer.compute(self.echo.dsp.as_mut(), output, 4);
            buffer.compute(self.reverb.dsp.as_mut(), output, 6);
        }

        // Inform UI of changed state keys
        let platform = self.platform.lock().unwrap();
        let send = move |change| platform.sender.send(change).unwrap();
        song.update_derived();
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

        // Steal voice with highest priority number, break ties with age
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
            voice.insert.dsp.set_param(song.note_index, note);
            voice.set_params(song, track_id);
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

#[derive(Clone)]
pub struct Controller {
    device: Device,
    song: Arc<RwLock<Song>>,
    audio: Arc<RwLock<Audio>>,
    sender: Arc<Mutex<Sender<(Command, i32)>>>,
}

impl Controller {
    pub fn stop(&self) {
        let _ = self.device.stop();
    }

    pub fn start(&self) {
        let _ = self.device.start();
    }

    pub fn load(&self, json: &Value) {
        self.stop();
        {
            let mut song = self.song.write().unwrap();
            let audio = self.audio.read().unwrap();
            let platform = audio.platform.lock().unwrap();
            *song = Song::new(&platform, &json);
            let dump = serde_json::to_value(song.dump()).unwrap();
            platform.sender.send(Change::Dump(dump)).unwrap();
        }
        self.start();
    }

    pub fn dump(&self) -> impl Serialize {
        self.song.read().unwrap().dump()
    }

    pub fn save(&self) -> impl Serialize {
        self.song.read().unwrap().save()
    }

    pub fn send(&self, method: &str, data: i32) {
        let callback = match method {
            "activeKey" => {
                Command::WithI32(|_, song, i| song.active_track().state.add("activeKey", i))
            }
            "activeTrack" => Command::WithUsize(|audio, song, i| {
                song.state.set("activeTrack", i);
                if !song.state.is("playing") {
                    let id = song.state.get("activeTrack");
                    audio.key_down(song, id, song.tracks[id].state.get("activeKey"));
                }
            }),
            "auditionDown" => Command::WithUsize(|audio, song, i| {
                audio.key_down(song, i, song.tracks[i].state.get("activeKey"))
            }),
            "auditionUp" => Command::WithUsize(|audio, song, i| {
                audio.key_up(song, i, song.tracks[i].state.get("activeKey"))
            }),
            "length" => Command::WithI32(|_, song, i| song.active_track().adjust_length(i)),
            "clear" => Command::WithUsize(|_, song, _| song.active_track().clear()),
            "muted" => Command::WithUsize(|_, song, i| song.tracks[i].state.toggle("muted")),
            "noteDown" => Command::WithUsize(|audio, song, i| {
                audio.key_down(song, song.state.get("activeTrack"), i)
            }),
            "noteUp" => Command::WithUsize(|audio, song, i| {
                audio.key_up(song, song.state.get("activeTrack"), i)
            }),
            "page" => Command::WithI32(|_, song, i| song.active_track().adjust_page(i)),
            "playing" => Command::WithUsize(|audio, song, _| audio.toggle_play(song)),
            "sampleType" => Command::WithUsize(|audio, song, i| audio.set_sample_type(song, i)),
            "sequence" => Command::WithUsize(|_, song, i| song.active_track().toggle_step(i)),
            "taps" => Command::WithUsize(|_, song, i| song.state.set("tempo", i)),
            "zoomIn" => Command::WithUsize(|_, song, _| song.active_track().zoom_in()),
            "zoomOut" => Command::WithUsize(|_, song, _| song.active_track().zoom_out()),
            _ => {
                let song = self.song.read().unwrap();
                if let Some(name) = song.state.find(method) {
                    Command::NudgeSong(name)
                } else if let Some(name) = song.active_track().state.find(method) {
                    Command::NudgeTrack(name)
                } else {
                    return;
                }
            }
        };
        let _ = self.sender.lock().unwrap().send((callback, data));
    }
}

pub fn init(platform: Platform, json: &Value) -> Result<Controller, Box<dyn Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let voice_count = platform.voice_count;
    let audio = Audio {
        platform: Mutex::new(platform),
        voices: vec![Voice::default(); voice_count],
        duck_mix: [0., 0.],
        echo: DspBox::<effects::echo>::default(),
        reverb: DspBox::<effects::reverb>::default(),
        receiver: Arc::new(Mutex::new(receiver)),
    };
    for v in audio.voices.iter() {
        assert_eq!(v.insert.dsp.get_num_inputs(), INSERT_INPUT_COUNT as i32);
        assert_eq!(v.insert.dsp.get_num_outputs(), INSERT_OUTPUT_COUNT as i32);
    }

    let mut device_config = DeviceConfig::new(DeviceType::Duplex);
    device_config.capture_mut().set_channels(1);
    device_config.capture_mut().set_format(Format::F32);
    device_config.playback_mut().set_channels(2);
    device_config.playback_mut().set_format(Format::F32);
    device_config.set_sample_rate(SAMPLE_RATE as u32);

    let song = Arc::new(RwLock::new(Song::new(
        &audio.platform.lock().unwrap(),
        &json,
    )));
    let audio = Arc::new(RwLock::new(audio));
    let controller_song = Arc::clone(&song);
    let controller_audio = Arc::clone(&audio);
    let mut device = Device::new(None, &device_config)?;
    device.set_data_callback(move |_, output, input| {
        audio
            .try_write()
            .unwrap()
            .process(&song.try_read().unwrap(), input, output);
    });

    Ok(Controller {
        device,
        song: controller_song,
        audio: controller_audio,
        sender: Arc::new(Mutex::new(sender)),
    })
}
