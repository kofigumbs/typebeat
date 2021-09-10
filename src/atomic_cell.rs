use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Small wrapper around crossbeam's AtomicCell, which adds Clone and serde support
#[derive(Default)]
pub struct AtomicCell<T>(crossbeam::atomic::AtomicCell<T>);

impl<T: Copy> Clone for AtomicCell<T> {
    fn clone(&self) -> Self {
        Self(self.load().into())
    }
}

impl<T> From<T> for AtomicCell<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> Deref for AtomicCell<T> {
    type Target = crossbeam::atomic::AtomicCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for AtomicCell<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(AtomicCell::from)
    }
}

impl<T: Copy + Serialize> Serialize for AtomicCell<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.load().serialize(serializer)
    }
}
