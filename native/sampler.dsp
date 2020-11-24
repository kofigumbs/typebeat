import("stdfaust.lib");

clamp(low, high) = min(high, max(low, _));

enfer(sample, position) = clamp(0, 255, sample), position : soundfile("enfer", 2) : untilEnd with {
	untilEnd(length, rate) = par(i, 2, *(position < length));
};

voices = 8*15;
process = par(i, voices, enfer) :> _, _;
