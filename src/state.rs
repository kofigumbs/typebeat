use std::collections::HashMap;
use std::marker::PhantomData;

use num_traits::AsPrimitive;
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

pub trait Host {
    fn host<F: FnMut(&'static str, &Param)>(f: &mut F);

    fn host_each<P, F: FnMut(&'static str, &P)>(f: &mut F, names: &'static [String], param: &P) {
        names.iter().for_each(|name| f(name, param));
    }
}

impl<H: FaustDsp<T = f32>> Host for H {
    fn host<F: FnMut(&'static str, &Param)>(f: &mut F) {
        struct ForEachEntry<F>(F);
        impl<F: FnMut(&'static str, &Param), P: AsPrimitive<i32>> UI<P> for ForEachEntry<F> {
            fn add_num_entry(&mut self, l: &'static str, _: ParamIndex, n: P, mi: P, ma: P, st: P) {
                self.0(l, Param::new(n).min(mi).max(ma).step(st));
            }
        }
        H::build_user_interface_static(&mut ForEachEntry(f));
    }
}

struct Slot {
    min: Option<i32>,
    max: Option<i32>,
    step: i32,
    value: AtomicCell<i32>,
    changed: AtomicCell<bool>,
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
    pub fn init(&self, value: &Value) {
        H::host(&mut |name, param| match value[name].as_i64() {
            Some(i) if !param.temp => self.set(name, i),
            _ => {}
        });
    }

    pub fn dump(&self) -> HashMap<&'static str, Value> {
        let mut output = HashMap::new();
        H::host(&mut |name, _param| {
            output.insert(name, self.get::<i32>(name).into());
        });
        output
    }

    pub fn save(&self) -> HashMap<&'static str, Value> {
        let mut output = HashMap::new();
        H::host(&mut |name, param| {
            let value = self.get::<i32>(name);
            if !param.temp && value != param.default {
                output.insert(name, value.into());
            }
        });
        output
    }

    pub fn for_each_change<F: FnMut(&'static str, i32)>(&self, mut f: F) {
        H::host(&mut |name, _param| {
            let slot = &self.slots[name];
            if slot.changed.swap(false) {
                f(name, slot.value.load());
            }
        });
    }
}

impl<H: Host> Default for State<H> {
    fn default() -> Self {
        let mut state = Self {
            slots: HashMap::default(),
            marker: PhantomData,
        };
        H::host(&mut |name, param| {
            state.slots.insert(
                name,
                Slot {
                    min: param.min,
                    max: param.max,
                    step: param.step,
                    value: param.default.into(),
                    changed: false.into(),
                },
            );
        });
        state
    }
}
