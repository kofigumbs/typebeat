use typebeat::{Controller, Platform};

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

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start() -> *const Controller {
    Box::leak(typebeat::start(StaticPlatform).expect("controller").into())
}

fn main() {}
