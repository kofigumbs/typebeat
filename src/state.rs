use std::collections::HashMap;
use std::ops::Deref;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;
use crate::effects::ParamIndex;

mod song {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/song.rs"));
}

mod track {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/track.rs"));
}

pub trait IsParam: Copy + Ord + PartialEq + Serialize + DeserializeOwned {
    fn to_i32(self) -> i32;
    fn from_i32(raw: i32) -> Self;
}

impl IsParam for bool {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn from_i32(raw: i32) -> Self {
        raw != 0
    }
}

impl IsParam for i32 {
    fn to_i32(self) -> i32 {
        self
    }
    fn from_i32(raw: i32) -> Self {
        raw
    }
}

impl IsParam for usize {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn from_i32(raw: i32) -> Self {
        raw as Self
    }
}

pub trait Enum: Sized + 'static {
    const ALL: &'static [Self];
}

impl<T: Copy + Ord + PartialEq + DeserializeOwned + Serialize + Enum> IsParam for T {
    fn to_i32(self) -> i32 {
        Self::ALL.iter().position(|&x| x == self).unwrap() as i32
    }
    fn from_i32(i: i32) -> Self {
        Self::ALL[i as usize]
    }
}

/// Tracks whether this parameter originated from a effects/*.dsp file
type DspId = Option<(&'static str, usize)>;

#[derive(Clone)]
pub struct Param<T: IsParam> {
    atom: AtomicCell<T>,
    default: T,
    min: Option<T>,
    max: Option<T>,
    step: Option<T>,
    ephemeral: bool,
    dsp_id: DspId,
    changed: AtomicCell<bool>,
}

impl<T: IsParam> Param<T> {
    pub fn new(
        default: T,
        min: Option<T>,
        max: Option<T>,
        step: Option<T>,
        ephemeral: bool,
        dsp_id: DspId,
    ) -> Self {
        Self {
            atom: default.into(),
            default,
            min,
            max,
            step,
            ephemeral,
            dsp_id,
            changed: false.into(),
        }
    }

    pub fn get(&self) -> T {
        self.atom.load()
    }

    pub fn set(&self, mut value: T) {
        if let Some(min) = self.min {
            value = T::max(min, value);
        }
        if let Some(max) = self.max {
            value = T::min(max, value);
        }
        if value != self.atom.swap(value) {
            self.changed.store(true);
        }
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self.get()).expect("value")
    }
}

impl Param<bool> {
    pub fn toggle(&self) {
        self.atom.toggle();
        self.changed.store(true);
    }
}

pub trait Visitor {
    fn call<P: IsParam>(&mut self, label: &'static str, param: &Param<P>);
}

struct ForEachDsp<T>(T);
impl<T: FnMut(&'static str, ParamIndex, f32)> Visitor for ForEachDsp<T> {
    fn call<P: IsParam>(&mut self, _: &'static str, param: &Param<P>) {
        match param.dsp_id.as_ref() {
            None => {}
            Some((name, i)) => self.0(name, ParamIndex(*i), param.get().to_i32() as f32),
        }
    }
}

struct ForEachChange<T>(T);
impl<T: FnMut(&'static str, Value)> Visitor for ForEachChange<T> {
    fn call<P: IsParam>(&mut self, label: &'static str, param: &Param<P>) {
        if param.changed.swap(false) {
            self.0(label, param.to_json());
        }
    }
}

struct Load(Value);
impl<'a> Visitor for Load {
    fn call<P: IsParam>(&mut self, label: &'static str, param: &Param<P>) {
        if let Ok(value) = serde_json::from_value(self.0[label].clone()) {
            param.set(value);
        }
    }
}

pub enum Format {
    Dump,
    Minimal,
}

struct Save<'a>(Format, &'a mut HashMap<&'static str, Value>);
impl<'a> Visitor for Save<'a> {
    fn call<P: IsParam>(&mut self, label: &'static str, param: &Param<P>) {
        match &self.0 {
            Format::Minimal if param.get() == param.default => None,
            Format::Minimal | Format::Dump => self.1.insert(label, param.to_json()),
        };
    }
}

pub trait IsState: Default {
    fn visit_params<T: Visitor>(&self, visitor: &mut T);
}

#[derive(Default)]
pub struct State<T>(T);

impl<T> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: IsState> State<T> {
    pub fn for_each_dsp<F: FnMut(&'static str, ParamIndex, f32)>(&self, f: F) {
        self.visit_params(&mut ForEachDsp(f));
    }

    pub fn for_each_change<F: FnMut(&'static str, Value)>(&self, f: F) {
        self.visit_params(&mut ForEachChange(f));
    }

    pub fn load(&self, value: Value) {
        self.visit_params(&mut Load(value));
    }

    pub fn save(&self, format: Format) -> impl Serialize {
        let mut bindings = HashMap::new();
        self.visit_params(&mut Save(format, &mut bindings));
        bindings
    }
}

pub type Song = State<song::State>;
pub type Track = State<track::State>;
