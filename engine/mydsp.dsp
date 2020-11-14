import("stdfaust.lib");
import("soundfiles.lib");

flip = xor(1, _);
frames_since(hold) = (hold*_)~+(1) : -(1) /* 0-index */;

playing = button("qKeyP") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("qSemicolon") : ba.toggle : hbargraph("armed", 0, 1);

bpm = 180;
beat = frames_since(playing) : int(_/ba.tempo(bpm)) % 16 : hbargraph("beat", 0, 15);

enfer = soundfile("enfer", 2) : !,!,_,_;
enfer_trigger(i, b) = i, frames_since(flip(ba.impulsify(b))) : enfer : init, init with {
	init = _ * ba.peakhold(1, b); // gate output until button is pressed once
};

process =
	enfer_trigger(0,  button("KeyN")),
	enfer_trigger(1,  button("KeyM")),
	enfer_trigger(2,  button("Comma")),
	enfer_trigger(3,  button("Period")),
	enfer_trigger(4,  button("Slash")),
	enfer_trigger(5,  button("KeyH")),
	enfer_trigger(6,  button("KeyJ")),
	enfer_trigger(7,  button("KeyK")),
	enfer_trigger(8,  button("KeyL")),
	enfer_trigger(9,  button("Semicolon")),
	enfer_trigger(10, button("KeyY")),
	enfer_trigger(11, button("KeyU")),
	enfer_trigger(12, button("KeyI")),
	enfer_trigger(13, button("KeyO")),
	enfer_trigger(14, button("KeyP")) :> _,_;
