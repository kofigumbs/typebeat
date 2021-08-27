use std::collections::HashMap;
use std::marker::PhantomData;

use crossbeam::atomic::AtomicCell;
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
    by: AtomicCell<i32>,
    default: AtomicCell<f32>,
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
            default: AtomicCell::new(0.),
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

    pub fn nudge_by(&self, by: i32) -> &Self {
        self.by.store(by);
        self
    }

    pub fn default(&self, default: T) -> &Self
    where
        T: Parameter,
    {
        self.default.store(default.to_f32());
        self
    }

    fn clone<U>(&self) -> Key<U> {
        Key {
            name: self.name,
            min: self.min.load().into(),
            max: self.max.load().into(),
            by: self.by.load().into(),
            default: self.default.load().into(),
            _marker: &PhantomData,
        }
    }
}

pub struct State {
    keys: HashMap<&'static str, Key<()>>,
    data: HashMap<&'static str, AtomicCell<f32>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

impl State {
    /// Read parameter from the saved value or use the default if it doesn't exist
    pub fn init<T: Copy + Parameter>(&mut self, save: &Value, key: &Key<T>) {
        let raw = if let Some(value) = save[key.name].as_f64() {
            value as f32
        } else {
            key.default.load()
        };
        self.keys.insert(key.name, key.clone());
        self.data.insert(key.name, AtomicCell::new(raw));
    }

    /// Gets the raw version of a parameter key by its name
    pub fn get_key<T>(&self, name: &str) -> Option<Key<T>> {
        self.keys.get(name).map(Key::clone)
    }

    /// Get the parameter's value
    pub fn get<T: Parameter>(&self, key: &Key<T>) -> T {
        Parameter::from_f32(self.data[key.name].load())
    }

    /// Set the parameter's value
    pub fn set<T: Copy + PartialOrd + Parameter>(&self, key: &Key<T>, value: T) {
        self.data[key.name].store(value.to_f32().clamp(key.min.load(), key.max.load()));
    }

    /// Toggles the boolean parameter's value
    pub fn toggle<T>(&self, key: &Key<T>) {
        let atom = &self.data[key.name];
        atom.store(bool::to_f32(atom.load() == 0.));
    }

    /// Toggles, sets, or nudges a parameter depending on the Key's properties
    pub fn nudge(&self, key: &Key<i32>, value: i32) {
        match (key.by.load(), value) {
            (0, _) => self.toggle(key),
            (1, _) => self.set(key, value),
            (x, 0) => self.set(key, self.get(key) - x),
            (_, 1) => self.set(key, self.get(key) - 1),
            (_, 2) => self.set(key, self.get(key) + 1),
            (x, 3) => self.set(key, self.get(key) + x),
            _ => {}
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, f32)> + '_ {
        let keys = &self.keys;
        self.data
            .iter()
            .map(|(&name, atom)| (name, atom.load()))
            .filter(move |(name, value)| *value != keys[name].default.load())
    }
}
