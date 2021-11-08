use std::collections::HashMap;
use std::marker::PhantomData;

use num_traits::AsPrimitive;
use serde::Serialize;
use serde_json::Value;

use crate::atomic_cell::AtomicCell;
use crate::effects::{FaustDsp, ParamIndex, UI};

pub struct Param {
    default: i32,
    min: Option<i32>,
    max: Option<i32>,
    step: i32,
    temp: bool,
}

impl Param {
    #[inline]
    pub fn new<P: AsPrimitive<i32>>(default: P) -> Param {
        Param {
            default: default.as_(),
            min: None,
            max: None,
            step: 1,
            temp: false,
        }
    }

    #[inline]
    pub fn min<P: AsPrimitive<i32>>(&mut self, min: P) -> &mut Self {
        self.min = Some(min.as_());
        self
    }

    #[inline]
    pub fn max<P: AsPrimitive<i32>>(&mut self, max: P) -> &mut Self {
        self.max = Some(max.as_());
        self
    }

    #[inline]
    pub fn step<P: AsPrimitive<i32>>(&mut self, step: P) -> &mut Self {
        self.step = step.as_();
        self
    }

    #[inline]
    pub fn toggle(&mut self) -> &mut Self {
        self.step = 0;
        self
    }

    #[inline]
    pub fn temp(&mut self) -> &mut Self {
        self.temp = true;
        self
    }
}

pub trait Visitor {
    fn visit(&mut self, name: &'static str, param: &Param);

    fn visit_each(&mut self, names: &'static [String], param: &Param) {
        names.iter().for_each(|name| self.visit(name, param));
    }
}

impl<V: Visitor, P: AsPrimitive<i32>> UI<P> for V {
    fn add_num_entry(&mut self, label: &'static str, _: ParamIndex, n: P, min: P, max: P, step: P) {
        self.visit(label, Param::new(n).min(min).max(max).step(step));
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

struct Slot {
    min: Option<i32>,
    max: Option<i32>,
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
    pub fn get<P: Copy + 'static>(&self, name: &'static str) -> P
    where
        i32: AsPrimitive<P>,
    {
        self.slots[name].value.load().as_()
    }

    pub fn is(&self, name: &'static str) -> bool {
        self.get::<i32>(name) != 0
    }

    pub fn set<P: AsPrimitive<i32>>(&self, name: &'static str, value: P) {
        let slot = &self.slots[name];
        let mut i = value.as_();
        if let Some(min) = slot.min {
            i = i32::max(i, min);
        }
        if let Some(max) = slot.max {
            i = i32::min(i, max);
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
            fn visit(&mut self, name: &'static str, param: &Param) {
                let value = self.1.get::<i32>(name);
                match &self.2 {
                    Strategy::Minimal if param.temp || value == param.default => None,
                    Strategy::Minimal | Strategy::Dump => self.0.insert(name, value.into()),
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
            fn visit(&mut self, name: &'static str, _: &Param) {
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
            fn visit(&mut self, name: &'static str, param: &Param) {
                self.0.entry(name).insert(Slot {
                    min: param.min,
                    max: param.max,
                    step: param.step,
                    value: param.default.into(),
                    changed: false.into(),
                });
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
