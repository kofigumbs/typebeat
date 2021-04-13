import("stdfaust.lib");

process = source : envelope : pan : volume;

source(input) = sampleSource with {
	sampleSource = sampleFile, position : soundfile("", 2) : !,!,_,_;
	position = ba.time * ba.semi2ratio(key) / ba.semi2ratio(naturalNote);
	//
	key = nentry("key", 0, 0, 127, 1);
	naturalNote = nentry("naturalNote", 69, 0, 127, 1);
	sampleFile = nentry("sampleFile", 0, 0, 14, 1);
};

envelope = sp.stereoize(*(gate)) with {
	gate = button("gate");
};

pan(inputL, inputR) = ba.select2stereo(amount > 0, toLeftL, toLeftR, toRightL, toRightR) with {
	toLeftL = inputL + inputR*abs(amount);
	toLeftR = inputR*(1+amount);
	toRightL = inputL*(1-amount);
	toRightR = inputR + inputL*amount;
	//
	amount = nentry("~pan", 25, 0, 50, 1) : /(25) : -(1);
};

volume = sp.stereoize(*(amount)) with {
	amount = nentry("~volume", 25, 0, 50, 1) : /(25);
};
