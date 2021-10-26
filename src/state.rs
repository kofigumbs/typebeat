use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::atomic_cell::AtomicCell;

pub enum Bind {
    Min(i32),
    Max(usize),
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

struct Value {
    value: AtomicCell<i32>,
    changed: AtomicCell<bool>,
}

pub struct State<T> {
    params: HashMap<&'static str, Value>,
    marker: PhantomData<fn() -> T>,
}

impl<H> State<H> {
    pub fn get<T: Param>(&self, key: &'static str) -> T {
        T::from_int(self.params[key].value.load())
    }

    pub fn set<T: Param>(&self, key: &'static str, value: T) {
        let value = value.to_int();
        let param = &self.params[key];
        if (value != param.value.swap(value)) {
            param.changed.store(true);
        }
    }

    pub fn toggle(&self, key: &'static str) {
        let param = &self.params[key];
        param.value.fetch_xor(1);
        param.changed.store(true);
    }
}

impl<H: Host> Default for State<H> {
    fn default() -> Self {
        struct V<'a>(&'a mut HashMap<&'static str, Value>);
        impl<'a> Visitor for V<'a> {
            fn visit<T: Param>(&mut self, name: &'static str, default: T, binds: &[Bind]) {
                let value = default.to_int().into();
                let changed = false.into();
                self.0.insert(name, Value { value, changed });
            }
        }
        let mut state = Self {
            params: HashMap::default(),
            marker: PhantomData,
        };
        H::host(&mut V(&mut state.params));
        state
    }
}
