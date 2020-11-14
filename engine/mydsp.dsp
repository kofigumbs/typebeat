import("stdfaust.lib");
import("soundfiles.lib");

note_count = 15;
notes   = par(i, note_count, button("note:%i"));
playing =                    button("play") : ba.toggle : hbargraph("playing", 0,  1);
armed   =                    button("arm")  : ba.toggle : hbargraph("armed",   0,  1);
beat    =                                                 hbargraph("beat",    0, 15);

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
enfer_trigger(i, b) = i, frames_since(trigger(b)) : enfer;

process = notes : par(i, note_count, _ : enfer_trigger(i)) :> _, _;
