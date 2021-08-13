use crossbeam::atomic::AtomicCell;
use num_traits::Num;

pub trait Bounded<T: Copy + PartialOrd + Num> {
    fn get_min(&self) -> T;
    fn get_max(&self) -> T;
    fn get_atom(&self) -> &AtomicCell<T>;

    fn load(&self) -> T {
        self.get_atom().load()
    }

    fn store(&self, value: T) {
        self.get_atom()
            .store(num_traits::clamp(value, self.get_min(), self.get_max()));
    }

    fn nudge(&self, value: i32, jump: T) {
        match value {
            0 => self.store(self.load() - jump),
            1 => self.store(self.load() - T::one()),
            2 => self.store(self.load() + T::one()),
            3 => self.store(self.load() + jump),
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct Int<const MIN: i32, const MAX: i32> {
    atom: AtomicCell<i32>,
}
impl<const MIN: i32, const MAX: i32> Bounded<i32> for Int<MIN, MAX> {
    fn get_min(&self) -> i32 {
        MIN
    }
    fn get_max(&self) -> i32 {
        MAX
    }
    fn get_atom(&self) -> &AtomicCell<i32> {
        &self.atom
    }
}
impl<const MIN: i32, const MAX: i32> From<i32> for Int<MIN, MAX> {
    fn from(value: i32) -> Self {
        Self { atom: value.into() }
    }
}

pub struct Dynamic<T> {
    pub atom: AtomicCell<T>,
    pub min: T,
    pub max: T,
}
impl<T: Copy + PartialOrd + Num> Bounded<T> for Dynamic<T> {
    fn get_min(&self) -> T {
        self.min
    }
    fn get_max(&self) -> T {
        self.max
    }
    fn get_atom(&self) -> &AtomicCell<T> {
        &self.atom
    }
}
