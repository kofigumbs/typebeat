use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Unifies <T: Copy> and AtomicCell<T: Copy>
pub trait CopyAs<T> {
    fn copy_as(&self) -> T;
}

impl<T: Copy> CopyAs<T> for T {
    fn copy_as(&self) -> T {
        *self
    }
}

/// Wrapper around crossbeam's AtomicCell, which adds Clone and serde support
#[derive(Default)]
pub struct AtomicCell<T>(crossbeam::atomic::AtomicCell<T>);

impl<T> From<T> for AtomicCell<T> {
    fn from(value: T) -> Self {
        assert!(crossbeam::atomic::AtomicCell::<T>::is_lock_free());
        Self(value.into())
    }
}

impl<T: Copy> Clone for AtomicCell<T> {
    fn clone(&self) -> Self {
        self.load().into()
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

impl<T: Copy> CopyAs<T> for AtomicCell<T> {
    fn copy_as(&self) -> T {
        self.load()
    }
}

impl AtomicCell<bool> {
    pub fn toggle(&self) {
        self.fetch_xor(true);
    }
}
