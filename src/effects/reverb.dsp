import("stdfaust.lib");

gain = nentry("reverbGain", 25, 0, 50, 10) : si.smoo;
comb = nentry("reverbComb", 25, 0, 50, 10) : si.smoo;
damp = nentry("reverbDamp", 25, 0, 50, 10);

process = sp.stereoize(*(gain/250)) : re.stereo_freeverb(0.7 + comb/175, allpass, damp/50, 23) with {
	allpass = 0.5;
};
