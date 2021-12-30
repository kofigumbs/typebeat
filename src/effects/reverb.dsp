import("stdfaust.lib");

gain = nentry("reverbGain", 25, 0, 50, 10) : si.smoo;
x    = nentry("reverbX",    25, 0, 50, 10) : si.smoo;
y    = nentry("reverbY",    25, 0, 50, 10);

process = sp.stereoize(*(gain/250)) : re.stereo_freeverb(comb, allpass, damp, 23) with {
	damp = y/50;
	comb = 0.7 + x/175;
	allpass = 0.5;
};
