import("stdfaust.lib");

scaleRoom    = nentry("scaleRoom",    14, 0, 50, 10) / 50;
offsetRoom   = nentry("offsetRoom",   35, 0, 50, 10) / 50;
allpassFeed  = nentry("allpassFeed",  25, 0, 50, 10) / 50;
scaleDamp    = nentry("scaleDamp",    20, 0, 50, 10) / 50;
fixedGain    = nentry("fixedGain",     5, 0, 50, 10) / 50;
damping      = nentry("damp",         25, 0, 50, 10) / 50;
roomSize     = nentry("roomSize",     25, 0, 50, 10) / 50;
stereoSpread = nentry("stereoSpread", 25, 0, 50, 10) / 50;

process = sp.stereoize(*(fixedGain)) : re.stereo_freeverb(
	roomSize * scaleRoom + offsetRoom,
	allpassFeed,
	damping * scaleDamp,
	int(stereoSpread * 46)
);
