use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use typebeat::{Controller, Platform};

extern "C" {
    pub fn typebeat_update(id: c_int, method: *const c_char, value: c_int);
}

struct App {
    controller: Controller,
    receiver: Mutex<Receiver<(usize, &'static str, i32)>>,
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

fn from_c_str(s: *const c_char) -> Option<&'static str> {
    unsafe { CStr::from_ptr(s.as_ref()?).to_str().ok() }
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
pub fn typebeat_send(method: *const c_char, data: i32) {
    APP.controller
        .send(from_c_str(method).expect("method"), data);
}

#[no_mangle]
pub fn typebeat_poll() -> *const c_char {
    let s = match APP.receiver.lock().expect("receiver").try_recv() {
        Ok((id, method, value)) => format!("{},{},{}", id, method, value),
        Err(_) => "".to_string(),
    };
    CString::new(s).expect("CString").into_raw()
}

fn main() {
    lazy_static::initialize(&APP);
}
