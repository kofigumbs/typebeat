import("stdfaust.lib");

note_count = 15;
notes = par(i, note_count, button("note:%i"));
playing = button("play") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("arm") : ba.toggle : hbargraph("armed", 0, 1);
trackType = nentry("setTrackType", 0, 0, 1, 1) : hbargraph("trackType", 0, 1);
instrument = nentry("setInstrument", 0, 0, 13, 1) : hbargraph("instrument", 0, 13);
beat = hbargraph("beat", 0, 15);

// ```
//             ┌──────────────────────────────┬────────────────┬───────┬──────────
// ────────────┘                              ╵                ╵       ╵
// ^ init: 0   ^ set to 1 on first trig (b)   ^ negative pulse on subsequent trigs
// ```
trigger(b) = flip(ba.impulsify(b)) * ba.peakhold(1, b);

// `0` to `1` and vice versa
flip = xor(1);

bpm = 180;
frames_since(hold) = (hold*_)~+(1) : _-1;
clock = frames_since(playing) : int(_/ba.tempo(bpm)) % 16 : beat;

enfer = soundfile("enfer", 2) : !,!,_,_;
enfer_kit_note(i) = ba.if(instrument == 13, i*18 + 15, i + instrument*18);
enfer_synth_note(i) = instrument * 16;
trigger_enfer(i, b) = min(i, 255), frames_since(trigger(b)) : enfer;
trigger_note(i, b) = enfer_kit_note(i), enfer_synth_note(i) : ba.selectn(2, trackType) : trigger_enfer(_, b);

process = notes : par(i, note_count, _ : trigger_note(i)) :> _, _;
