import("stdfaust.lib");

gain  = nentry("reverb:gain",  25, 0, 50, 10) : si.smoo;
damp  = nentry("reverb:damp",  25, 0, 50, 10) : si.smoo;
width = nentry("reverb:width", 25, 0, 50, 10);

process = sp.stereoize(*(gain/50)) : re.stereo_freeverb(comb, allpass, damp/100, width) with {
	comb = 0.23;
	allpass = 0.5;
};
