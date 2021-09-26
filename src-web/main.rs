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

lazy_static::lazy_static! {
    static ref CONTROLLER: Controller = typebeat::init(WebPlatform).expect("controller");
}

fn c_str(chars: *const c_char) -> Option<&'static str> {
    unsafe { CStr::from_ptr(chars.as_ref()?).to_str().ok() }
}

#[no_mangle]
pub fn start() {
    CONTROLLER.start();
}

#[no_mangle]
pub fn stop() {
    CONTROLLER.stop();
}

#[no_mangle]
pub fn get(method: *const c_char) -> i32 {
    c_str(method)
        .and_then(|method| CONTROLLER.get(method))
        .unwrap_or_default()
}

#[no_mangle]
pub fn set(method: *const c_char, data: i32) {
    c_str(method).map(|method| CONTROLLER.set(method, data));
}

pub fn main() {
    lazy_static::initialize(&CONTROLLER);
}
