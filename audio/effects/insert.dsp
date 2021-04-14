import("stdfaust.lib");

gate = button("gate[hidden:xx]");
key = nentry("key[hidden:xx]", 0, 0, 127, 1);
sampleFile = nentry("sampleFile[hidden:xx]", 0, 0, 14, 1);
naturalNote = nentry("naturalNote", 69, 0, 127, 1);
attack = nentry("attack", 0, 0, 50, 1);
sustain = nentry("sustain", 50, 0, 50, 1);
decay = nentry("decay", 0, 0, 50, 1);
release = nentry("release", 0, 0, 50, 1);
volume = nentry("volume", 25, 0, 50, 1);
pan = nentry("pan", 25, 0, 50, 1);

process = source : envelope : mix;

source(input) = sampleSource with {
	sampleSource = sound(slow), sound(fast) :> par(i, 2, it.interpolate_linear(position - slow));
	sound = sampleFile, _ : soundfile("", 2) : !,!,_,_;
	slow = floor(position);
	fast = ceil(position);
	position = ba.time * ba.semi2ratio(key) / ba.semi2ratio(naturalNote);
};

envelope = sp.stereoize(*(en.adsr(attack/20, decay/20, sustain/50, release/20, gate)));

mix(inputL, inputR) = panned : sp.stereoize(*(volume/25)) with {
	panned = ba.select2stereo(panAmount > 0, toLeftL, toLeftR, toRightL, toRightR);
	panAmount = pan/25 - 1;
	toLeftL = inputL + inputR*abs(panAmount);
	toLeftR = inputR*(1+panAmount);
	toRightL = inputL*(1-panAmount);
	toRightR = inputR + inputL*panAmount;
};
