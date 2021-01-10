import("stdfaust.lib");

bi = sp.stereoize;
clamp(low, high) = min(high, max(low, _));

stereoPan(amount, inputL, inputR) = ba.select2stereo(amount < 0,
	inputL + inputR*abs(amount),  inputR*(1+amount),
	inputL*(1-amount),            inputR + inputL*amount
);

effect(controls) = stereoPan(pan) : bi(*(velocity)) <: bi(_), bi(*(reverb)) with {
	velocity  = controlValue(0,  0, 1);
	pan       = controlValue(1, -1, 1);
	filter    = controlValue(2, -1, 1);
	resonance = controlValue(3,  0, 1);
	reverb    = controlValue(4,  0, 1);
	delay     = controlValue(5,  0, 1);
	controlValue(n, low, high) = ((controls >> (n*4)) & 15) / 14, low, high : it.interpolate_cosine;
};

enfer(sample, position, controls) = clamp(0, 255, sample), position : soundfile("enfer", 2) : untilEnd : effect(controls) with {
	untilEnd(length, rate) = bi(*(position < length));
};

voices = 8*15;
process = par(i, voices, enfer) :> bi(_), dm.freeverb_demo :> bi(_);
