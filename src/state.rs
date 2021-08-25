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
#[derive(Default)]
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

    /// Keys for use with State#get
    pub fn read_only(name: &'static str) -> Self
    where
        T: Default,
    {
        Self {
            name,
            default: T::default(),
            range: None,
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
pub struct State<Aux> {
    map: HashMap<&'static str, (AtomicCell<f32>, Option<Aux>)>,
}

impl<Aux> State<Aux> {
    /// Read parameter from the saved value or use the default if it doesn't exist
    pub fn init<T: Copy + Parameter>(&mut self, key: &Key<T>, saved: &Value) {
        let raw = if let Some(value) = saved[key.name].as_f64() {
            value as f32
        } else {
            key.default.to_f32()
        };
        self.map.insert(key.name, (AtomicCell::new(raw), None));
    }

    /// Attaches aux data to an existing key
    pub fn with_aux<T>(&mut self, key: &Key<T>, aux: Aux) {
        self.map
            .entry(key.name)
            .and_modify(|pair| pair.1 = Some(aux));
    }

    /// Gets the static name for a parameter if it exists
    pub fn get_name(&self, name: &str) -> Option<&'static str> {
        self.map.get_key_value(name).map(|(&name, _)| name)
    }

    /// Get the parameter's aux association
    pub fn get_aux<T>(&self, key: &Key<T>) -> &Aux {
        self.map[key.name].1.as_ref().unwrap()
    }

    /// Get the parameter's value
    pub fn get<T: Parameter>(&self, key: &Key<T>) -> T {
        Parameter::from_f32(self.map[key.name].0.load())
    }

    /// Set the parameter's value
    pub fn set<T: Copy + PartialOrd + Parameter>(&self, key: &Key<T>, value: T) {
        self.map[key.name].0.store(match &key.range {
            None => value.to_f32(),
            Some(range) => num_traits::clamp(value, *range.start(), *range.end()).to_f32(),
        });
    }

    /// Toggles the boolean parameter's value
    pub fn toggle<T>(&self, key: &Key<T>) {
        let atom = &self.map[key.name].0;
        atom.store(bool::to_f32(atom.load() == 0.));
    }

    /// Update the parameter's value by +/- 1 or jump, depending on the the value provided
    pub fn nudge<T: Copy + PartialOrd + Num + Parameter>(&self, key: &Key<T>, value: i32, jump: T) {
        match value {
            0 => self.set(key, self.get(key) - jump),
            1 => self.set(key, self.get(key) - T::one()),
            2 => self.set(key, self.get(key) + T::one()),
            3 => self.set(key, self.get(key) + jump),
            _ => {}
        }
    }
}
