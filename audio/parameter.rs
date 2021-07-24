use std::sync::atomic::Ordering;

use atomic_float::AtomicF32;

pub struct Parameter {
    min: f32,
    max: f32,
    value: AtomicF32,
}

impl Parameter {
    pub fn binary(value: bool) -> Self {
        Parameter {
            min: 0.,
            max: 1.,
            value: if value { 1.0.into() } else { 0.0.into() },
        }
    }

    pub fn new(value: f32) -> Self {
        Parameter {
            min: f32::MIN,
            max: f32::MAX,
            value: AtomicF32::new(value),
        }
    }

    pub fn between<T: Into<f32>>(self, min: T, max: T) -> Self {
        Parameter {
            min: min.into(),
            max: max.into(),
            value: self.value,
        }
    }

    pub fn get(&self) -> f32 {
        self.value.load(Ordering::Relaxed).into()
    }

    pub fn is_zero(&self) -> bool {
        self.get() == 0.
    }

    pub fn set<T: Into<f32>>(&self, value: T) {
        self.value
            .store(value.into().clamp(self.min, self.max), Ordering::Relaxed);
    }

    pub fn toggle(&self) {
        self.set(if self.is_zero() { 1. } else { 0. });
    }

    pub fn nudge(&self, data: u8, jump: f32) {
        let diff = match data {
            0 => -jump,
            1 => -1.,
            2 => 1.,
            3 => jump,
            _ => 0.,
        };
        self.set(diff + self.get());
    }
}
