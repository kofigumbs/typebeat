use serde_json::Value;

pub fn save() -> Value {
    serde_json::json!(
        {"song":{},"tracks":[{"sequence":[],"live":""},{"sequence":[],"live":""},{"sequence":[],"live":""},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","sequence":[]},{"sequence":[{"hit":162,"key":14,"step":136},{"hit":135,"key":14,"step":424}],"resolution":64,"pan":5,"activeKey":14,"live":""},{"sequence":[],"live":""},{"sequence":[],"live":""},{"sequence":[],"live":""},{"live":"","sequence":[]},{"pan":-10,"live":"","reverb":6,"sequence":[{"hit":32,"key":12,"step":64}],"echo":6,"main":0,"length":1024},{"live":"","sequence":[]},{"live":"","sequence":[]},{"live":"","cutoff":10,"drive":6,"release":16,"highFreq":46,"decay":30,"synth2Detune":-12,"synth2Type":2,"highRes":-4,"synth1Level":30,"synth3Type":4,"length":2048,"resolution":64,"activeKey":9,"sequence":[{"hit":56,"key":12,"step":128},{"hit":64,"key":12,"step":232},{"hit":51,"key":11,"step":328},{"hit":51,"key":10,"step":432},{"hit":58,"key":9,"step":656},{"hit":53,"key":9,"step":752},{"hit":56,"key":8,"step":848},{"hit":56,"key":7,"step":936},{"hit":56,"key":12,"step":1168},{"hit":214,"key":12,"step":1264},{"hit":74,"key":10,"step":1664},{"hit":257,"key":9,"step":1760}],"sustain":30,"reverb":3,"synth2Level":26,"main":20,"sampleLevel":0,"attack":1,"octave":2}]}
    )
}
