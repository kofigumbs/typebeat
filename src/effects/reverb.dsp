import("stdfaust.lib");

gain  = nentry("reverb gain",  25, 0, 50, 10) : si.smoo;
feed  = nentry("reverb feed",  25, 0, 50, 10) : si.smoo;
space = nentry("reverb space", 25, 0, 50, 10);

process = sp.stereoize(*(gain/250)) : re.stereo_freeverb(comb, allpass, damp, space) with {
	damp = 0.2;
	comb = 0.7 + feed/175;
	allpass = 0.5;
};
