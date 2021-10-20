use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;

use num_traits::{AsPrimitive, Num, One, Zero};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;
pub use crate::effects::{FaustDsp, ParamIndex};

pub mod song {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/song.rs"));
}

pub mod track {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/track.rs"));
}

pub trait IsParam: Any + Copy + PartialOrd + PartialEq + DeserializeOwned + Serialize {
    fn nudge(self, data: u8, step: u8) -> Self;
}

impl IsParam for bool {
    fn nudge(self, _: u8, _: u8) -> Self {
        !self
    }
}

pub trait IsNumParam: Any + Copy + PartialOrd + PartialEq + DeserializeOwned + Serialize {}
impl IsNumParam for f32 {}
impl IsNumParam for i32 {}
impl IsNumParam for usize {}

impl<T: Num + IsNumParam> IsParam for T
where
    u8: AsPrimitive<T>,
{
    fn nudge(self, data: u8, step: u8) -> Self {
        match data {
            _ if step.is_zero() => T::one() - self,
            _ if step.is_one() => data.as_(),
            0 => self - step.as_(),
            1 => self - T::one(),
            2 => self + T::one(),
            3 => self + step.as_(),
            _ => self,
        }
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
    step: u8,
    ephemeral: bool,
    dsp_id: DspId,
    changed: AtomicCell<bool>,
}

impl<T: IsParam> Param<T> {
    pub fn new(
        default: T,
        min: Option<T>,
        max: Option<T>,
        step: u8,
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

    pub fn nudge(&self, data: u8) {
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
                <dyn Any>::downcast_ref(&param.get())
                    .map(|value| self.2.set_param(ParamIndex(*i), *value));
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

pub enum Format {
    Dump,
    Minimal,
}

struct Save<'a, S>(&'a S, Format, &'a mut HashMap<&'static str, Value>);
impl<'a, S> Visitor<S> for Save<'a, S> {
    fn visit<P: IsParam>(&mut self, label: &'static str, get_param: fn(&S) -> &Param<P>) {
        let param = get_param(&self.0);
        match &self.1 {
            Format::Minimal if param.ephemeral || param.get() == param.default => None,
            Format::Minimal | Format::Dump => self.2.insert(label, param.to_json()),
        };
    }
}

pub trait IsState: Default {
    fn visit_params<V: Visitor<Self>>(visitor: &mut V);
}

#[derive(Default)]
pub struct State<S>(S);

impl<S> Deref for State<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S: IsState + 'static> State<S> {
    fn nudges() -> HashMap<&'static str, Box<dyn Fn(&S, Value) + Sync>> {
        let mut bindings = HashMap::new();
        S::visit_params(&mut Nudges(&mut bindings));
        bindings
    }

    pub fn set_params(&self, dsp_name: &'static str, dsp: &mut dyn FaustDsp<T = f32>) {
        S::visit_params(&mut SetParams(&self.0, dsp_name, dsp));
    }

    pub fn for_each_change(&self, f: impl FnMut(&'static str, Value)) {
        S::visit_params(&mut ForEachChange(&self.0, f));
    }

    pub fn load(&self, value: Value) {
        S::visit_params(&mut Load(&self.0, value));
    }

    pub fn save(&self, format: Format) -> impl Serialize {
        let mut bindings = HashMap::new();
        S::visit_params(&mut Save(&self.0, format, &mut bindings));
        bindings
    }
}

pub type Song = State<song::State>;
pub type Track = State<track::State>;

lazy_static::lazy_static! {
    pub static ref SONG_NUDGES: HashMap<&'static str, Box<dyn Fn(&song::State, Value) + Sync>> = Song::nudges();
    pub static ref TRACK_NUDGES: HashMap<&'static str, Box<dyn Fn(&track::State, Value) + Sync>> = Track::nudges();
}
