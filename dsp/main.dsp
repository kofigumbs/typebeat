import("stdfaust.lib");

process = ba.beat(180) : ba.pulse_countup_loop(15, 1) : hbargraph("beat", 0, 15);
