use std::collections::HashMap;

use num_traits::AsPrimitive;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;
pub use crate::effects::{FaustDsp, ParamIndex};

pub trait IsParam:
    Copy + PartialOrd + PartialEq + DeserializeOwned + Serialize + AsPrimitive<i32>
{
    fn nudge(self, data: i32, step: i32) -> Self;
}

impl IsParam for bool {
    fn nudge(self, _: i32, _: i32) -> Self {
        !self
    }
}

impl IsParam for i32 {
    fn nudge(self, data: i32, step: i32) -> Self {
        match data {
            _ if step == 0 => self ^ 1,
            _ if step == 1 => data,
            0 => self.saturating_sub(step),
            1 => self.saturating_sub(1),
            2 => self.saturating_add(1),
            3 => self.saturating_add(step),
            _ => self,
        }
    }
}

impl IsParam for usize {
    fn nudge(self, data: i32, step: i32) -> Self {
        (self as i32).nudge(data, step) as usize
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
    step: i32,
    ephemeral: bool,
    dsp_id: DspId,
    changed: AtomicCell<bool>,
}

impl<T: IsParam> Param<T> {
    pub fn new(
        default: T,
        min: Option<T>,
        max: Option<T>,
        step: i32,
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
            value = num_traits::clamp_min(value, min);
        }
        if let Some(max) = self.max {
            value = num_traits::clamp_max(value, max);
        }
        if value != self.atom.swap(value) {
            self.changed.store(true);
        }
    }

    pub fn nudge(&self, data: i32) {
        self.set(self.get().nudge(data, self.step))
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

pub trait Visitor<S> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>);
}

struct SetParams<'a, S>(&'a S, &'static str, &'a mut dyn FaustDsp<T = f32>);
impl<'a, S> Visitor<S> for SetParams<'a, S> {
    fn visit<P: IsParam>(&mut self, _: &'static str, get_param: fn(&S) -> &Param<P>) {
        let param = get_param(self.0);
        match param.dsp_id.as_ref() {
            Some((name, i)) if name == &self.1 => {
                self.2.set_param(ParamIndex(*i), param.get().as_() as f32)
            }
            _ => {}
        }
    }
}

struct ForEachChange<'a, S, F>(&'a S, F);
impl<'a, S, F: FnMut(&'static str, Value)> Visitor<S> for ForEachChange<'a, S, F> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>) {
        let param = get_param(self.0);
        if param.changed.swap(false) {
            self.1(label, param.to_json());
        }
    }
}

struct Nudges<'a, S: 'static>(&'a mut HashMap<&'static str, Box<dyn Fn(&S, Value) + Sync>>);
impl<'a, S> Visitor<S> for Nudges<'a, S> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>) {
        self.0.insert(
            label,
            Box::new(move |state, value| match serde_json::from_value(value) {
                Ok(value) => get_param(state).nudge(value),
                Err(_) => {}
            }),
        );
    }
}

struct Load<'a, S>(&'a S, Value);
impl<'a, S> Visitor<S> for Load<'a, S> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>) {
        let param = get_param(&self.0);
        match serde_json::from_value(self.1[label].clone()) {
            Ok(value) if !param.ephemeral => param.set(value),
            _ => {}
        }
    }
}

pub enum Strategy {
    Dump,
    Minimal,
}

struct Save<'a, S>(&'a S, Strategy, &'a mut HashMap<&'static str, Value>);
impl<'a, S> Visitor<S> for Save<'a, S> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>) {
        let param = get_param(&self.0);
        match &self.1 {
            Strategy::Minimal if param.ephemeral || param.get() == param.default => None,
            Strategy::Minimal | Strategy::Dump => self.2.insert(label, param.to_json()),
        };
    }
}

pub trait IsState: Default + 'static {
    fn visit_params<V: Visitor<Self>>(visitor: &mut V);

    fn nudges() -> HashMap<&'static str, Box<dyn Fn(&Self, Value) + Sync>> {
        let mut bindings = HashMap::new();
        Self::visit_params(&mut Nudges(&mut bindings));
        bindings
    }

    fn set_params(&self, dsp_name: &'static str, dsp: &mut dyn FaustDsp<T = f32>) {
        Self::visit_params(&mut SetParams(self, dsp_name, dsp));
    }

    fn for_each_change<F: FnMut(&'static str, Value)>(&self, f: F) {
        Self::visit_params(&mut ForEachChange(self, f));
    }

    fn load(&self, value: Value) {
        Self::visit_params(&mut Load(self, value));
    }

    fn save(&self, strategy: Strategy) -> HashMap<&'static str, Value> {
        let mut bindings = HashMap::new();
        Self::visit_params(&mut Save(self, strategy, &mut bindings));
        bindings
    }
}

include!(concat!(env!("OUT_DIR"), "/song.rs"));
include!(concat!(env!("OUT_DIR"), "/track.rs"));

lazy_static::lazy_static! {
    pub static ref SONG_NUDGES: HashMap<&'static str, Box<dyn Fn(&song, Value) + Sync>> = song::nudges();
    pub static ref TRACK_NUDGES: HashMap<&'static str, Box<dyn Fn(&track, Value) + Sync>> = track::nudges();
}
