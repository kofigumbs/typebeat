import("stdfaust.lib");

process = source : pan : volume;

source(input) = sample, position : soundfile("", 2) : untilEnd with {
	note = nentry("note", 0, 0, 127, 1);
	sample = nentry("sample", 0, 0, 14, 1);
	position = framesSince(note != note');
	untilEnd(length, rate) = sp.stereoize(*(position < length));
};

volume = sp.stereoize(*(amount)) with {
	amount = nentry("volume", 25, 0, 50, 1) : /(50) : *(2) : si.smoo;
};

pan(inputL, inputR) = ba.select2stereo(amount > 0, toLeftL, toLeftR, toRightL, toRightR) with {
	amount = nentry("pan", 25, 0, 50, 1) : /(25) : -(1) : si.smoo;
	toLeftL = inputL + inputR*abs(amount);
	toLeftR = inputR*(1+amount);
	toRightL = inputL*(1-amount);
	toRightR = inputR + inputL*amount;
};

clamp(low, high) = min(high, max(low, _));
framesSince(hold) = (hold*_) ~ +(1) : -(hold);
