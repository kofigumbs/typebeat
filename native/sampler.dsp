import("stdfaust.lib");

clamp(low, high) = min(high, max(low, _));
input(s, n) = ((s >> (n*4 + 8)) & 15) / 14;

// https://gist.github.com/sletz/1008c384394a9883110be3ae83e20d17
stereoPan(amount, inputL, inputR) =
		ba.if((amount <= 0), inputL + inputR * gainL, inputL * gainL),
		ba.if((amount <= 0), inputR * gainR, inputR + inputL * gainR) with {
	amount1 = ba.if(amount <= 0, amount + 1, amount);
	gainL = cos(amount1 * ma.PI / 2);
	gainR = sin(amount1 * ma.PI / 2);
};

effect(s) = stereoPan(pan) : sp.stereoize(*(velocity)) with {
	velocity = input(s, 0);
	pan = input(s, 1) : _, -1, 1 : it.interpolate_linear;
	filter = input(s, 2);
	resonance = input(s, 3);
	reverb = input(s, 4);
	delay = input(s, 5);
};

enfer(s, position) = clamp(0, 255, sample), position : soundfile("enfer", 2) : untilEnd : effect(s) with {
	sample = s & 255;
	untilEnd(length, rate) = par(i, 2, *(position < length));
};

voices = 8*15;
process = par(i, voices, enfer) :> _, _;
