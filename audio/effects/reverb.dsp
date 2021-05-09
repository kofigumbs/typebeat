import("stdfaust.lib");

process = sp.stereoize(*(fixedgain)) : re.stereo_freeverb(combfeed, allpassfeed, damping, spatSpread) with {
	scaleroom   = 0.28;
	offsetroom  = 0.7;
	allpassfeed = 0.5;
	scaledamp   = 0.4;
	fixedgain   = 0.1;
	damping     = (.5)*scaledamp;
	combfeed    = (.5)*scaleroom + offsetroom;
	spatSpread  = (.5)*46 : int;
};
