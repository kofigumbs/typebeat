use std::sync::atomic::{AtomicPtr, Ordering};

use typebeat::{Controller, Platform};

static CONTROLLER: AtomicPtr<Controller> = AtomicPtr::new(std::ptr::null_mut());

struct StaticPlatform;

impl Platform for StaticPlatform {
    fn get_stereo_sample(&self, _i: usize) -> Vec<f32> {
        let count: usize = 65536;
        let mut table = Vec::with_capacity(count);
        for i in 0..count {
            let mut x = [0, 0];
            let mut y = [0, 0];
            x[0] = 1;
            y[0] = (x[1] + y[1]) % 65536;
            table[i as usize] = f32::sin(9.58738019e-05 * (y[0] as f32));
            x[1] = x[0];
            y[1] = y[0];
        }
        table
    }
}

#[no_mangle]
pub fn set(method: &str, data: i32) {
    unsafe { &*CONTROLLER.load(Ordering::Relaxed) }.set(method, data)
}

#[no_mangle]
pub fn get(method: &str) -> i32 {
    unsafe { &*CONTROLLER.load(Ordering::Relaxed) }
        .get(method)
        .unwrap_or_default()
}

pub fn main() {
    CONTROLLER.store(
        Box::leak(typebeat::start(StaticPlatform).expect("controller").into()),
        Ordering::Relaxed,
    );
}
