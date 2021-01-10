import("stdfaust.lib");

bi = sp.stereoize;
clamp(low, high) = min(high, max(low, _));
input(c, n) = ((c >> (n*4)) & 15) / 14;

// https://gist.github.com/sletz/1008c384394a9883110be3ae83e20d17
stereoPan(amount, inputL, inputR) =
		ba.if(toL, inputL + inputR * gainL, inputL * gainL),
		ba.if(toL, inputR * gainR, inputR + inputL * gainR) with {
	toL = amount <= .5;
	gainL = cos(x * ma.PI / 2);
	gainR = sin(x * ma.PI / 2);
	x = ba.if(toL, amount*2, amount*2 - 1);
};

effect(controls) = stereoPan(pan) : bi(*(velocity)) <: bi(_), bi(*(reverb)) with {
	velocity = input(controls, 0);
	pan = input(controls, 1);
	filter = input(controls, 2);
	resonance = input(controls, 3);
	reverb = input(controls, 4);
	delay = input(controls, 5);
};

enfer(sample, position, controls) = clamp(0, 255, sample), position : soundfile("enfer", 2) : untilEnd : effect(controls) with {
	untilEnd(length, rate) = bi(*(position < length));
};

voices = 8*15;
process = par(i, voices, enfer) :> bi(_), dm.freeverb_demo :> bi(_);
