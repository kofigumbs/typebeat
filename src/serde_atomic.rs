use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<AtomicCell<T>, D::Error> {
    T::deserialize(deserializer).map(AtomicCell::new)
}

pub fn serialize<S: Serializer, T: Copy + Serialize>(
    atom: &AtomicCell<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    atom.load().serialize(serializer)
}
