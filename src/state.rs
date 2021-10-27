use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;
use crate::effects::{FaustDsp, ParamIndex, UI};

pub enum Bind {
    Max(usize),
    Min(i32),
    Step(i32),
    Temp,
}

pub trait Visitor {
    fn visit<P: Param>(&mut self, name: &'static str, default: P, binds: &[Bind]);
}

impl<V: Visitor, P: Param> UI<P> for V {
    fn add_num_entry(&mut self, label: &'static str, _: ParamIndex, n: P, min: P, max: P, step: P) {
        let binds = &[
            Bind::Max(max.to_int() as usize),
            Bind::Min(min.to_int()),
            Bind::Step(step.to_int()),
        ];
        self.visit(label, n, binds);
    }
}

pub trait Host {
    fn host<V: Visitor>(visitor: &mut V);
}

impl<T: FaustDsp<T = f32>> Host for T {
    fn host<V: Visitor>(visitor: &mut V) {
        Self::build_user_interface_static(visitor);
    }
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

    fn transcode<P: Param>(value: P) -> Self {
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

pub enum Strategy {
    Dump,
    Minimal,
}

pub struct State<T> {
    slots: HashMap<&'static str, Slot>,
    marker: PhantomData<fn() -> T>,
}

impl<H> State<H> {
    pub fn get<P: Param>(&self, name: &'static str) -> P {
        P::from_int(self.slots[name].value.load())
    }

    pub fn set<P: Param>(&self, name: &'static str, value: P) {
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

    pub fn add(&self, name: &'static str, i: i32) {
        self.set(name, self.get::<i32>(name).saturating_add(i));
    }

    pub fn nudge(&self, name: &'static str, data: i32) {
        let slot = &self.slots[name];
        match data {
            _ if slot.step == 0 => self.toggle(name),
            _ if slot.step == 1 => self.set(name, data),
            0 => self.add(name, -slot.step),
            1 => self.add(name, -1),
            2 => self.add(name, 1),
            3 => self.add(name, slot.step),
            _ => {}
        }
    }

    pub fn find(&self, name: &str) -> Option<&'static str> {
        self.slots.get_key_value(name).map(|pair| *pair.0)
    }
}

impl<H: Host> State<H> {
    pub fn save(&self, strategy: Strategy) -> impl Serialize {
        struct Guest<'a, S>(&'a mut HashMap<&'static str, Value>, &'a State<S>, Strategy);
        impl<'a, S: Host> Visitor for Guest<'a, S> {
            fn visit<P: Param>(&mut self, name: &'static str, default: P, binds: &[Bind]) {
                let param = self.1.get::<i32>(name);
                let temp = binds.iter().any(|bind| matches!(bind, Bind::Temp));
                match &self.2 {
                    Strategy::Minimal if temp || param == default.to_int() => None,
                    Strategy::Minimal | Strategy::Dump => self.0.insert(name, param.into()),
                };
            }
        }
        let mut slots = HashMap::default();
        H::host(&mut Guest(&mut slots, self, strategy));
        slots
    }

    pub fn for_each_change<F: FnMut(&'static str, i32)>(&self, f: F) {
        struct Guest<'a, F, S>(F, &'a S);
        impl<'a, F: FnMut(&'static str, i32), S> Visitor for Guest<'a, F, State<S>> {
            fn visit<P: Param>(&mut self, name: &'static str, _: P, _: &[Bind]) {
                let slot = &self.1.slots[name];
                if slot.changed.swap(false) {
                    self.0(name, slot.value.load());
                }
            }
        }
        H::host(&mut Guest(f, self));
    }
}

impl<H: Host> Default for State<H> {
    fn default() -> Self {
        struct Guest<'a>(&'a mut HashMap<&'static str, Slot>);
        impl<'a> Visitor for Guest<'a> {
            fn visit<P: Param>(&mut self, name: &'static str, default: P, binds: &[Bind]) {
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
        H::host(&mut Guest(&mut state.slots));
        state
    }
}
