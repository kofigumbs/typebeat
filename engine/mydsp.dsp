import("stdfaust.lib");
import("soundfiles.lib");

playing = button("qp") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("q;") : ba.toggle : hbargraph("armed", 0, 1);

bpm = 180;
frames_since_play = (playing*_)~+(1) : -(1) /* 0-index */;
beat = frames_since_play : int(_/ba.tempo(bpm)) % 16 : hbargraph("beat", 0, 15);

enfer = soundfile("enfer[url:enfer.wav]", 2) : !,!,_,_;

process =
	(0, (button("n")*_)~+(1) : enfer),
	(1, (button("m")*_)~+(1) : enfer),
	(2, (button(",")*_)~+(1) : enfer),
	// (3, (button(".")*_)~+(1) : enfer), // crashes compiler
	// (4, (button("/")*_)~+(1) : enfer), // no sound: https://faustdoc.grame.fr/manual/syntax/#labels-as-pathnames
	(5, (button("h")*_)~+(1) : enfer),
	(6, (button("j")*_)~+(1) : enfer),
	(7, (button("k")*_)~+(1) : enfer),
	(8, (button("l")*_)~+(1) : enfer),
	(9, (button(";")*_)~+(1) : enfer),
	(10, (button("y")*_)~+(1) : enfer),
	(11, (button("u")*_)~+(1) : enfer),
	(12, (button("i")*_)~+(1) : enfer),
	(13, (button("o")*_)~+(1) : enfer),
	(14, (button("p")*_)~+(1) : enfer) :> _,_;
