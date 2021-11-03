use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use serde::Serialize;

use typebeat::{Change, Controller, Platform};

struct App {
    controller: Controller,
    receiver: Mutex<Receiver<Change>>,
}

lazy_static::lazy_static! {
    static ref APP: App = {
        let voice_count = 4;
        let (sender, receiver) = std::sync::mpsc::channel();
        let root = PathBuf::from("/assets");
        App {
            receiver: Mutex::new(receiver),
            controller: typebeat::init(Platform { voice_count, sender, root }).expect("controller"),
        }
    };
}

fn from_c_str(s: *const c_char) -> &'static str {
    unsafe { CStr::from_ptr(s).to_str().expect("CStr") }
}

fn to_c_str_json<T: Serialize>(s: T) -> *const c_char {
    let json = serde_json::to_string(&s).expect("json");
    CString::new(json).expect("CString").into_raw()
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
    let receiver = APP.receiver.lock().expect("receiver");
    to_c_str_json(receiver.try_iter().collect::<Vec<_>>())
}

fn main() {
    lazy_static::initialize(&APP);
}
