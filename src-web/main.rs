use typebeat::{Controller, Platform};

struct StaticPlatform;

impl Platform for StaticPlatform {
    fn get_stereo_sample(&self, _i: usize) -> Vec<f32> {
        Vec::new() // TODO
    }
}

#[no_mangle]
pub fn set(controller: *const Controller, method: &str, data: i32) {
    unsafe { &*controller }.set(method, data)
}

#[no_mangle]
pub fn get(controller: *const Controller, method: &str) -> i32 {
    unsafe { &*controller }.get(method).unwrap_or_default()
}

#[no_mangle]
pub fn start() -> *const Controller {
    Box::leak(typebeat::start(StaticPlatform).expect("controller").into())
}

pub fn main() {}
