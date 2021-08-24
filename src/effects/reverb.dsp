import("stdfaust.lib");

gain  = nentry("reverbGain",  25, 0, 50, 10) : si.smoo;
feed  = nentry("reverbFeed",  25, 0, 50, 10) : si.smoo;
space = nentry("reverbSpace", 25, 0, 50, 10);

process = sp.stereoize(*(gain/250)) : re.stereo_freeverb(comb, allpass, damp, space) with {
	damp = 0.2;
	comb = 0.7 + feed/175;
	allpass = 0.5;
};
