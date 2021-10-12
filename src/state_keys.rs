use crate::state::Key;
use crate::{SampleType, View};

// Primitive song keys
pub static CAN_CLEAR: Key<bool> = Key::new("canClear");
pub static PLAYING: Key<bool> = Key::new("playing");
pub static RECORDING: Key<bool> = Key::new("recording");
pub static TEMPO: Key<i32> = Key::new("tempo");
pub static ROOT: Key<i32> = Key::new("root");
pub static SCALE: Key<usize> = Key::new("scale");
pub static ACTIVE_TRACK_ID: Key<usize> = Key::new("activeTrack");

// Primitive track keys
pub static MUTED: Key<bool> = Key::new("muted");
pub static USE_KEY: Key<bool> = Key::new("useKey");
pub static ACTIVE_KEY: Key<i32> = Key::new("activeKey");
pub static OCTAVE: Key<i32> = Key::new("octave");
pub static LENGTH: Key<usize> = Key::new("length");
pub static RESOLUTION: Key<usize> = Key::new("resolution");
pub static BARS: Key<usize> = Key::new("bars");
pub static VIEW_START: Key<usize> = Key::new("viewStart");
pub static PAGE_START: Key<usize> = Key::new("pageStart");
pub static RECENT: Key<usize> = Key::new("recent");
pub static SAMPLE_DETUNE: Key<f32> = Key::new("sampleDetune"); // registered by dsp
pub static SAMPLE_TYPE: Key<SampleType> = Key::new("sampleType"); // registered by dsp


// Slice track keys

static NOTE_0: Key<i32> = Key::new("note 0");
static NOTE_1: Key<i32> = Key::new("note 1");
static NOTE_2: Key<i32> = Key::new("note 2");
static NOTE_3: Key<i32> = Key::new("note 3");
static NOTE_4: Key<i32> = Key::new("note 4");
static NOTE_5: Key<i32> = Key::new("note 5");
static NOTE_6: Key<i32> = Key::new("note 6");
static NOTE_7: Key<i32> = Key::new("note 7");
static NOTE_8: Key<i32> = Key::new("note 8");
static NOTE_9: Key<i32> = Key::new("note 9");
static NOTE_10: Key<i32> = Key::new("note 10");
static NOTE_11: Key<i32> = Key::new("note 11");
static NOTE_12: Key<i32> = Key::new("note 12");
static NOTE_13: Key<i32> = Key::new("note 13");
static NOTE_14: Key<i32> = Key::new("note 14");
pub static NOTES: &[&Key<i32>] = &[
    &NOTE_0, &NOTE_1, &NOTE_2, &NOTE_3, &NOTE_4, &NOTE_5, &NOTE_6, &NOTE_7, &NOTE_8, &NOTE_9,
    &NOTE_10, &NOTE_11, &NOTE_12, &NOTE_13, &NOTE_14,
];

static VIEW_0: Key<View> = Key::new("view 0");
static VIEW_1: Key<View> = Key::new("view 1");
static VIEW_2: Key<View> = Key::new("view 2");
static VIEW_3: Key<View> = Key::new("view 3");
pub static VIEWS: &[&Key<View>] = &[&VIEW_0, &VIEW_1, &VIEW_2, &VIEW_3];

static WAVEFORM_0: Key<f32> = Key::new("waveform 0");
static WAVEFORM_1: Key<f32> = Key::new("waveform 1");
static WAVEFORM_2: Key<f32> = Key::new("waveform 2");
static WAVEFORM_3: Key<f32> = Key::new("waveform 3");
static WAVEFORM_4: Key<f32> = Key::new("waveform 4");
static WAVEFORM_5: Key<f32> = Key::new("waveform 5");
static WAVEFORM_6: Key<f32> = Key::new("waveform 6");
static WAVEFORM_7: Key<f32> = Key::new("waveform 7");
static WAVEFORM_8: Key<f32> = Key::new("waveform 8");
static WAVEFORM_9: Key<f32> = Key::new("waveform 9");
static WAVEFORM_10: Key<f32> = Key::new("waveform 10");
static WAVEFORM_11: Key<f32> = Key::new("waveform 11");
static WAVEFORM_12: Key<f32> = Key::new("waveform 12");
static WAVEFORM_13: Key<f32> = Key::new("waveform 13");
static WAVEFORM_14: Key<f32> = Key::new("waveform 14");
static WAVEFORM_15: Key<f32> = Key::new("waveform 15");
static WAVEFORM_16: Key<f32> = Key::new("waveform 16");
static WAVEFORM_17: Key<f32> = Key::new("waveform 17");
static WAVEFORM_18: Key<f32> = Key::new("waveform 18");
static WAVEFORM_19: Key<f32> = Key::new("waveform 19");
static WAVEFORM_20: Key<f32> = Key::new("waveform 20");
static WAVEFORM_21: Key<f32> = Key::new("waveform 21");
static WAVEFORM_22: Key<f32> = Key::new("waveform 22");
static WAVEFORM_23: Key<f32> = Key::new("waveform 23");
pub static WAVEFORMS: &[&Key<f32>] = &[
    &WAVEFORM_0,
    &WAVEFORM_1,
    &WAVEFORM_2,
    &WAVEFORM_3,
    &WAVEFORM_4,
    &WAVEFORM_5,
    &WAVEFORM_6,
    &WAVEFORM_7,
    &WAVEFORM_8,
    &WAVEFORM_9,
    &WAVEFORM_10,
    &WAVEFORM_11,
    &WAVEFORM_12,
    &WAVEFORM_13,
    &WAVEFORM_14,
    &WAVEFORM_15,
    &WAVEFORM_16,
    &WAVEFORM_17,
    &WAVEFORM_18,
    &WAVEFORM_19,
    &WAVEFORM_20,
    &WAVEFORM_21,
    &WAVEFORM_22,
    &WAVEFORM_23,
];
