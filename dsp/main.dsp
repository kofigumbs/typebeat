import("stdfaust.lib");

playing = button("qp") : ba.toggle : hbargraph("playing", 0, 1);
process = ba.beat(180) : ba.pulse_countup_loop(15, playing) : hbargraph("beat", 0, 15);
