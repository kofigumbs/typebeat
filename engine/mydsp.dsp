import("stdfaust.lib");

playing = button("qp") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("q;") : ba.toggle : hbargraph("armed", 0, 1);

bpm = 180;
frames_since_play = (playing*_)~+(1) : -(1) /* 0-index */;
beat = frames_since_play : int(_/ba.tempo(bpm)) % 16 : hbargraph("beat", 0, 15);

enfer_sample_count = 234;

// process = beat, armed : max;
process = 0, _~+(1) : soundfile("enfer[url:enfer.wav]", enfer_sample_count) : ba.selector(2, enfer_sample_count + 2);
