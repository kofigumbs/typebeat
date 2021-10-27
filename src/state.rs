use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;

pub enum Bind {
    Max(usize),
    Min(i32),
    Step(i32),
    Temp,
}

pub trait Visitor {
    fn visit<T: Param>(&mut self, name: &'static str, default: T, binds: &[Bind]);
}

pub trait Host {
    fn host<T: Visitor>(visitor: &mut T);
}

pub trait Param: DeserializeOwned + Serialize {
    fn to_int(&self) -> i32 {
        serde_json::to_value(self)
            .ok()
            .and_then(|value| value.as_i64())
            .expect("Param::to_int") as i32
    }

    fn from_int(value: i32) -> Self {
        serde_json::from_value(value.into()).expect("Param::from_int")
    }

    fn transcode<T: Param>(value: T) -> Self {
        Self::from_int(value.to_int())
    }
}

impl Param for bool {
    fn to_int(&self) -> i32 {
        return if *self { 1 } else { 0 };
    }

    fn from_int(value: i32) -> Self {
        value != 0
    }
}

impl Param for i32 {
    fn to_int(&self) -> i32 {
        *self
    }

    fn from_int(value: i32) -> Self {
        value
    }
}

impl Param for f32 {
    fn to_int(&self) -> i32 {
        *self as i32
    }

    fn from_int(value: i32) -> Self {
        value as Self
    }
}

impl Param for usize {
    fn to_int(&self) -> i32 {
        *self as i32
    }

    fn from_int(value: i32) -> Self {
        value as Self
    }
}

struct Slot {
    max: Option<i32>,
    min: Option<i32>,
    step: i32,
    value: AtomicCell<i32>,
    changed: AtomicCell<bool>,
}

enum Strategy {
    Dump,
    Minimal,
}

pub struct State<T> {
    slots: HashMap<&'static str, Slot>,
    marker: PhantomData<fn() -> T>,
}

impl<H: Host> State<H> {
    pub fn get<T: Param>(&self, name: &'static str) -> T {
        T::from_int(self.slots[name].value.load())
    }

    pub fn set<T: Param>(&self, name: &'static str, value: T) {
        let slot = &self.slots[name];
        let mut i = value.to_int();
        if let Some(max) = slot.max {
            i = i32::min(i, max);
        }
        if let Some(min) = slot.min {
            i = i32::max(i, min);
        }
        if i != slot.value.swap(i) {
            slot.changed.store(true);
        }
    }

    pub fn toggle(&self, name: &'static str) {
        let slot = &self.slots[name];
        slot.value.fetch_xor(1);
        slot.changed.store(true);
    }

    pub fn nudge(&self, name: &'static str, data: i32) {
        let slot = &self.slots[name];
        let i = slot.value.load();
        match data {
            _ if slot.step == 0 => self.toggle(name),
            _ if slot.step == 1 => self.set(name, data),
            0 => self.set(name, i.saturating_sub(slot.step)),
            1 => self.set(name, i.saturating_sub(1)),
            2 => self.set(name, i.saturating_add(1)),
            3 => self.set(name, i.saturating_add(slot.step)),
            _ => {}
        }
    }

    pub fn find(&self, name: &str) -> Option<&'static str> {
        self.slots.get_key_value(name).map(|pair| *pair.0)
    }

    pub fn save(&self, strategy: Strategy) -> impl Serialize {
        struct V<'a, S>(&'a mut HashMap<&'static str, Value>, &'a State<S>, Strategy);
        impl<'a, S: Host> Visitor for V<'a, S> {
            fn visit<T: Param>(&mut self, name: &'static str, default: T, binds: &[Bind]) {
                let param = self.1.get::<i32>(name);
                let temp = binds.iter().any(|bind| matches!(bind, Bind::Temp));
                match &self.2 {
                    Strategy::Minimal if temp || param == default.to_int() => None,
                    Strategy::Minimal | Strategy::Dump => self.0.insert(name, param.into()),
                };
            }
        }
        let mut slots = HashMap::default();
        H::host(&mut V(&mut slots, self, strategy));
        slots
    }
}

impl<H: Host> Default for State<H> {
    fn default() -> Self {
        struct V<'a>(&'a mut HashMap<&'static str, Slot>);
        impl<'a> Visitor for V<'a> {
            fn visit<T: Param>(&mut self, name: &'static str, default: T, binds: &[Bind]) {
                let mut slot = Slot {
                    max: None,
                    min: None,
                    step: 1,
                    value: default.to_int().into(),
                    changed: false.into(),
                };
                binds.iter().for_each(|bind| match bind {
                    Bind::Max(u) => slot.max = Some(*u as i32),
                    Bind::Min(i) => slot.min = Some(*i),
                    Bind::Step(i) => slot.step = *i,
                    Bind::Temp => {}
                });
                self.0.insert(name, slot);
            }
        }
        let mut state = Self {
            slots: HashMap::default(),
            marker: PhantomData,
        };
        H::host(&mut V(&mut state.slots));
        state
    }
}
