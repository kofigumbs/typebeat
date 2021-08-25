use std::collections::HashMap;
use std::marker::PhantomData;

use crossbeam::atomic::AtomicCell;
use num_traits::{AsPrimitive, Num};
use serde_json::Value;

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

/// Tags a parameter
///   1. Parameters are bidirectionally convertible to f32s
///   2. Parameters are clamped
///   3. Parameters are persisted in the save file by name
///   4. Parameters are available to the front-end by name
/// <https://matklad.github.io/2018/05/24/typed-key-pattern.html>
pub struct Key<T: 'static> {
    name: &'static str,
    min: AtomicCell<f32>,
    max: AtomicCell<f32>,
    by: AtomicCell<u8>,
    _marker: &'static PhantomData<T>,
}

impl<T> Key<T> {
    /// Defines a persisted, unbounded parameter tag
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            min: AtomicCell::new(f32::MIN),
            max: AtomicCell::new(f32::MAX),
            by: AtomicCell::new(1),
            _marker: &PhantomData,
        }
    }

    pub fn between(&self, min: T, max: T) -> &Self
    where
        T: Parameter,
    {
        self.min.store(min.to_f32());
        self.max.store(max.to_f32());
        self
    }

    pub fn nudge_by(&self, by: u8) -> &Self {
        self.by.store(by);
        self
    }
}

pub struct State<Aux> {
    map: HashMap<&'static str, (AtomicCell<f32>, Option<Aux>)>,
}

impl<Aux> Default for State<Aux> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<Aux> State<Aux> {
    /// Read parameter from the saved value or use the default if it doesn't exist
    pub fn init<T: Copy + Parameter>(&mut self, saved: &Value, key: &Key<T>, default: T) {
        let raw = if let Some(value) = saved[key.name].as_f64() {
            value as f32
        } else {
            default.to_f32()
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
        self.map[key.name]
            .0
            .store(value.to_f32().clamp(key.min.load(), key.max.load()));
    }

    /// Toggles the boolean parameter's value
    pub fn toggle<T>(&self, key: &Key<T>) {
        let atom = &self.map[key.name].0;
        atom.store(bool::to_f32(atom.load() == 0.));
    }

    /// Update the parameter's value by +/- 1 or by, depending on the the value provided
    pub fn nudge<T: Copy + PartialOrd + Num + Parameter>(&self, key: &Key<T>, value: i32, by: T) {
        match value {
            0 => self.set(key, self.get(key) - by),
            1 => self.set(key, self.get(key) - T::one()),
            2 => self.set(key, self.get(key) + T::one()),
            3 => self.set(key, self.get(key) + by),
            _ => {}
        }
    }

    /// Toggles, sets, or nudges a parameter depending on the Key's properties
    pub fn update<T: AsPrimitive<f32> + AsPrimitive<i32>>(&self, key: &Key<f32>, value: T) {
        match key.by.load() {
            0 => self.toggle(key),
            1 => self.set(key, value.as_()),
            by => self.nudge(key, value.as_(), by as f32),
        }
    }
}
