use std::ffi::CStr;
use std::os::raw::c_char;

use typebeat::{Controller, Platform};

struct StaticPlatform;

impl Platform for StaticPlatform {
    fn get_stereo_sample(&self, _i: usize) -> Vec<f32> {
        Vec::new() // TODO
    }
}

fn c_str(chars: *const c_char) -> Option<&'static str> {
    unsafe { chars.as_ref().map(|chars| CStr::from_ptr(chars)) }
        .and_then(|chars| chars.to_str().ok())
}

#[no_mangle]
pub fn set(controller: *const Controller, method: *const c_char, data: i32) {
    unsafe { controller.as_ref() }
        .zip(c_str(method))
        .map(|(controller, method)| controller.set(method, data));
}

#[no_mangle]
pub fn get(controller: *const Controller, method: *const c_char) -> i32 {
    unsafe { controller.as_ref() }
        .zip(c_str(method))
        .and_then(|(controller, method)| controller.get(method))
        .unwrap_or_default()
}

#[no_mangle]
pub fn start() -> *const Controller {
    Box::leak(typebeat::start(StaticPlatform).expect("controller").into())
}

pub fn main() {}
