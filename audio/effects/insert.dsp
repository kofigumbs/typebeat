import("stdfaust.lib");

e = environment {
	gate = button("gate[hidden:xx]");
	key = nentry("key[hidden:xx]", 0, 0, 127, 1);
	sampleFile = nentry("sampleFile[hidden:xx]", 0, 0, 14, 1);
	naturalNote = nentry("naturalNote", 69, 0, 127, 1);
	volume = nentry("volume", 25, 0, 50, 1);
	pan = nentry("pan", 25, 0, 50, 1);
};

process = source : envelope : mix;

source(input) = sampleSource with {
	sampleSource = e.sampleFile, position : soundfile("", 2) : !,!,_,_;
	position = ba.time * ba.semi2ratio(e.key) / ba.semi2ratio(e.naturalNote);
};

envelope = sp.stereoize(*(e.gate));

mix(inputL, inputR) = pan : sp.stereoize(*(e.volume/25)) with {
	pan = ba.select2stereo(panAmount > 0, toLeftL, toLeftR, toRightL, toRightR);
	panAmount = e.pan/25 - 1;
	toLeftL = inputL + inputR*abs(panAmount);
	toLeftR = inputR*(1+panAmount);
	toRightL = inputL*(1-panAmount);
	toRightR = inputR + inputL*panAmount;
};
