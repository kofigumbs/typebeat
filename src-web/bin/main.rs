use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

use typebeat::{Controller, Platform};

struct WebPlatform;

impl Platform for WebPlatform {
    fn root(&self) -> &Path {
        Path::new("/src")
    }
}

fn c_str(chars: *const c_char) -> Option<&'static str> {
    unsafe { CStr::from_ptr(chars.as_ref()?).to_str().ok() }
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
    Box::leak(typebeat::start(WebPlatform).expect("controller").into())
}

#[no_mangle]
pub fn stop(controller: *const Controller) {
    unsafe { controller.as_ref() }.map(|controller| controller.stop());
}

pub fn main() {}
