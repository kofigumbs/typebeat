use std::collections::HashMap;
use std::marker::PhantomData;

use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Serialize, Serializer};

pub trait Parameter {
    fn to_i32(self) -> i32;
    fn from_i32(raw: i32) -> Self;
}

impl Parameter for bool {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn from_i32(raw: i32) -> Self {
        raw != 0
    }
}

impl Parameter for i32 {
    fn to_i32(self) -> i32 {
        self
    }
    fn from_i32(raw: i32) -> Self {
        raw
    }
}

impl Parameter for f32 {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn from_i32(raw: i32) -> Self {
        raw as f32
    }
}

impl Parameter for usize {
    fn to_i32(self) -> i32 {
        self as i32
    }
    fn from_i32(raw: i32) -> Self {
        raw as usize
    }
}

pub trait Enum: Sized + 'static {
    const ALL: &'static [Self];
}

impl<T: Copy + PartialEq + Enum> Parameter for T {
    fn to_i32(self) -> i32 {
        Self::ALL.iter().position(|&x| x == self).unwrap() as i32
    }
    fn from_i32(raw: i32) -> Self {
        Self::ALL[(raw as usize).min(Self::ALL.len() - 1)]
    }
}

/// Tags a parameter
///   1. Parameters are bidirectionally convertible to i32s
///   2. Parameters are clamped
///   3. Parameters are persisted in the save file by name
///   4. Parameters are available to the front-end by name
/// <https://matklad.github.io/2018/05/24/typed-key-pattern.html>
pub struct Key<T: 'static> {
    name: &'static str,
    min: AtomicCell<i32>,
    max: AtomicCell<i32>,
    by: AtomicCell<i32>,
    default: AtomicCell<i32>,
    _marker: &'static PhantomData<T>,
}

impl<T> Key<T> {
    /// Defines a persisted, unbounded parameter tag
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            min: AtomicCell::new(i32::MIN),
            max: AtomicCell::new(i32::MAX),
            by: AtomicCell::new(1),
            default: AtomicCell::new(0),
            _marker: &PhantomData,
        }
    }

    pub fn between(&self, min: T, max: T) -> &Self
    where
        T: Parameter,
    {
        self.min.store(min.to_i32());
        self.max.store(max.to_i32());
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
        self.default.store(default.to_i32());
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

struct Meta {
    key: Key<()>,
    dirty: AtomicCell<bool>,
}

#[derive(Default, Deserialize)]
pub struct State {
    #[serde(default, flatten)]
    save: HashMap<String, i32>,
    #[serde(skip)]
    meta: HashMap<&'static str, Meta>,
    #[serde(skip)]
    data: HashMap<&'static str, AtomicCell<i32>>,
}

impl<'a> From<HashMap<&'a str, i32>> for State {
    fn from(save: HashMap<&'a str, i32>) -> Self {
        Self {
            save: save
                .into_iter()
                .map(|(name, value)| (name.to_owned(), value))
                .collect(),
            ..Self::default()
        }
    }
}

/// Serialize to save file
impl Serialize for State {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_save().serialize(serializer)
    }
}

impl State {
    /// Read parameter from the saved value or use the default if it doesn't exist
    pub fn register<T: Copy + Parameter>(&mut self, key: &Key<T>) {
        let data = self.save.remove(key.name).unwrap_or(key.default.load());
        let meta = Meta {
            key: key.clone(),
            dirty: false.into(),
        };
        self.meta.insert(key.name, meta);
        self.data.insert(key.name, data.into());
    }

    /// Gets the raw version of a parameter key by its name
    pub fn get_key<T>(&self, name: &str) -> Option<Key<T>> {
        self.meta.get(name).map(|meta| meta.key.clone())
    }

    /// Get the parameter's value
    pub fn get<T: Parameter>(&self, key: &Key<T>) -> T {
        Parameter::from_i32(self.data[key.name].load())
    }

    /// Set the parameter's value
    pub fn set<T: Copy + PartialOrd + Parameter>(&self, key: &Key<T>, value: T) {
        let new = value.to_i32().clamp(key.min.load(), key.max.load());
        let old = self.data[key.name].swap(new);
        if new != old {
            self.meta[key.name].dirty.store(true);
        }
    }

    /// Toggles the boolean parameter's value
    pub fn toggle<T>(&self, key: &Key<T>) {
        self.data[key.name].fetch_xor(1);
        self.meta[key.name].dirty.store(true);
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

    /// Iterates through dirty state key names
    pub fn dirty(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.meta
            .iter()
            .filter_map(|(&name, meta)| meta.dirty.swap(false).then_some(name))
    }

    /// Formats state for saving
    pub fn to_save(&self) -> HashMap<&'static str, i32> {
        self.data
            .iter()
            .map(|(&name, atom)| (name, atom.load()))
            .filter(move |(name, value)| *value != self.meta[name].key.default.load())
            .collect()
    }
}
