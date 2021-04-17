import("stdfaust.lib");

gate = button("gate");
sampleLevel = nentry("sample:level", 50, 0, 50, 1);
samplePitch = nentry("sample:level", 25, 0, 50, 1);
synth1Level = nentry("synth:1:level", 0, 0, 50, 1);
synth1Pitch = nentry("synth:1:pitch", 25, 0, 50, 1);
synth2Level = nentry("synth:2:level", 0, 0, 50, 1);
synth2Pitch = nentry("synth:2:pitch", 25, 0, 50, 1);
attack = nentry("attack", 0, 0, 50, 1);
sustain = nentry("sustain", 50, 0, 50, 1);
decay = nentry("decay", 0, 0, 50, 1);
release = nentry("release", 1, 0, 50, 1);
volume = nentry("volume", 25, 0, 50, 1);
pan = nentry("pan", 25, 0, 50, 1);

process = source : envelope : mix;

source = sample, synth1, synth2 :> _, _ with {
	sample = sp.stereoize(*(sampleLevel/50));
	synth1 = os.oscsin(ba.midikey2hz(69 - 25 + synth1Pitch)) : *(synth1Level/50) <: _, _;
	synth2 = os.oscsin(ba.midikey2hz(69 - 25 + synth2Pitch)) : *(synth2Level/50) <: _, _;
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
