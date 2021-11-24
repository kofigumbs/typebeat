use serde_json::Value;

pub fn save() -> Value {
    serde_json::json!(
        {"song":{"version":1},"tracks":[{"sequence":[],"live":""},{"sequence":[],"live":""},{"live":"","sequence":[]},{"live":"","sequence":[]},{"sequence":[],"live":""},{"live":"","sequence":[{"hit":32,"key":12,"step":128},{"hit":32,"key":12,"step":384}]},{"live":"","sequence":[]},{"sequence":[],"live":""},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","sequence":[]},{"resolution":4,"sequence":[{"hit":128,"key":5,"step":0},{"hit":128,"key":9,"step":128},{"hit":128,"key":7,"step":256},{"hit":128,"key":6,"step":384}],"synth2Level":10,"synth1Level":20,"highFreq":45,"sustain":40,"synth2Detune":-10,"synth2Type":3,"octave":3,"release":5,"live":"","decay":10,"activeKey":9,"highRes":-10}]}
    )
}
