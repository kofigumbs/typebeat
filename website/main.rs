use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use serde::Serialize;

use typebeat::{Change, Controller, Platform};

mod demo;

struct App {
    controller: Controller,
    receiver: Mutex<Receiver<Change>>,
}

lazy_static::lazy_static! {
    static ref APP: App = {
        let (sender, receiver) = std::sync::mpsc::channel();
        let root = PathBuf::from("/");
        let save = &demo::save();
        App {
            receiver: Mutex::new(receiver),
            controller: typebeat::init(Platform { voice_count: 2, sender, root }, save).unwrap(),
        }
    };
}

/// Convert C string (from Emscripten) to Rust str
fn from_c_str(s: *const c_char) -> &'static str {
    unsafe { CStr::from_ptr(s).to_str().unwrap() }
}

/// Convert Rust type to C string (form Emscripten) as JSON
fn to_c_str_json<T: Serialize>(s: T) -> *const c_char {
    let json = serde_json::to_string(&s).unwrap();
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub fn typebeat_start() {
    APP.controller.start();
}

#[no_mangle]
pub fn typebeat_stop() {
    APP.controller.stop();
}

#[no_mangle]
pub fn typebeat_dump() -> *const c_char {
    to_c_str_json(APP.controller.dump())
}

#[no_mangle]
pub fn typebeat_send(method: *const c_char, data: i32) {
    APP.controller.send(from_c_str(method), data);
}

#[no_mangle]
pub fn typebeat_changes() -> *const c_char {
    let receiver = APP.receiver.lock().unwrap();
    to_c_str_json(receiver.try_iter().collect::<Vec<_>>())
}

fn main() {
    // Eagerly initialize the static ref so that dump is immediate
    lazy_static::initialize(&APP);
}
