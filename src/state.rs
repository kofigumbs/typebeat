use std::collections::HashMap;
use std::ops::RangeInclusive;

use crossbeam::atomic::AtomicCell;
use num_traits::Num;
use serde_json::Value;

/// Optional clamping boundaries for a Key
pub type Range<T> = Option<RangeInclusive<T>>;

/// Tags a parameter
///   1. Parameters are bidirectionally convertible to f32s
///   2. Parameters are persisted in the save file
///   3. Parameters are optionally clamped
///   4. Parameters have a default value
/// <https://matklad.github.io/2018/05/24/typed-key-pattern.html>
pub struct Key<T> {
    name: &'static str,
    default: T,
    range: Range<T>,
}

impl<T> Key<T> {
    /// Defines a parameter tag
    pub const fn new(name: &'static str, default: T, range: Range<T>) -> Self {
        Self {
            name,
            default,
            range,
        }
    }
}

impl Key<bool> {
    /// Defines a boolean parameter tag, defaulting to false
    pub const fn toggle(name: &'static str) -> Self {
        Self::new(name, false, None)
    }
}

pub trait Parameter {
    fn to_f32(self) -> f32;
    fn from_f32(raw: f32) -> Self;
}

impl Parameter for bool {
    fn to_f32(self) -> f32 {
        return if self { 1. } else { 0. };
    }
    fn from_f32(raw: f32) -> Self {
        raw == 1.
    }
}

impl Parameter for usize {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(raw: f32) -> Self {
        raw as usize
    }
}

impl Parameter for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(raw: f32) -> Self {
        raw as i32
    }
}

impl Parameter for f32 {
    fn to_f32(self) -> f32 {
        self
    }
    fn from_f32(raw: f32) -> Self {
        raw
    }
}

pub trait Enum: Sized + 'static {
    const ALL: &'static [Self];
}

impl<T: Copy + PartialEq + Enum> Parameter for T {
    fn to_f32(self) -> f32 {
        Self::ALL.iter().position(|&x| x == self).unwrap() as f32
    }
    fn from_f32(raw: f32) -> Self {
        Self::ALL[(raw as usize).min(Self::ALL.len() - 1)]
    }
}

#[derive(Default)]
pub struct State {
    map: HashMap<&'static str, AtomicCell<f32>>,
}

impl State {
    /// Read parameter from the saved value or use the default if it doesn't exist
    pub fn deserialize<T: Copy + Parameter>(&mut self, key: &Key<T>, saved: &Value) {
        if let Some(value) = saved[key.name].as_f64() {
            self.map.insert(key.name, AtomicCell::new(value as f32));
        } else {
            self.map.insert(key.name, key.default.to_f32().into());
        }
    }

    /// Get the parameter's value
    pub fn get<T: Parameter>(&self, key: &Key<T>) -> T {
        Parameter::from_f32(self.map[key.name].load())
    }

    /// Set the parameter's value
    pub fn set<T: Copy + Ord + Parameter>(&self, key: &Key<T>, value: T) {
        self.map[key.name].store(match &key.range {
            None => value.to_f32(),
            Some(range) => value.clamp(*range.start(), *range.end()).to_f32(),
        });
    }

    /// Toggles the boolean parameter's value
    pub fn toggle(&self, key: &Key<bool>) {
        self.set(key, !self.get(key));
    }

    /// Update the parameter's value by +/- 1 or jump, depending on the the value provided
    pub fn nudge<T: Copy + Ord + Num + Parameter>(&self, key: &Key<T>, value: i32, jump: T) {
        match value {
            0 => self.set(key, self.get(key) - jump),
            1 => self.set(key, self.get(key) - T::one()),
            2 => self.set(key, self.get(key) + T::one()),
            3 => self.set(key, self.get(key) + jump),
            _ => {}
        }
    }
}
