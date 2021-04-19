import("stdfaust.lib");

note = button("note");
gate = button("gate");

synth1Type = nentry("synth1:type", 0, 0, 4, 1);
synth2Type = nentry("synth2:type", 0, 0, 4, 1);
adsrSample = nentry("adsrSample",   0, 0, 1, 0);

samplePitch = nentry("sample:pitch",  0, -120, 120, 10) : smooth;
sampleLevel = nentry("sample:level", 50,    0,  50, 10) : smooth;
synth1Pitch = nentry("synth1:pitch",  0, -120, 120, 10) : smooth;
synth1Level = nentry("synth1:level",  0,    0,  50, 10) : smooth;
synth2Pitch = nentry("synth2:pitch",  0, -120, 120, 10) : smooth;
synth2Level = nentry("synth2:level",  0,    0,  50, 10) : smooth;
lowFreq     = nentry("low:freq",      0,    0,  50, 10) : smooth;
lowRes      = nentry("low:res",       0,  	0,  50, 10) : smooth;
mid1Freq    = nentry("mid1:freq",     0,    0,  50, 10) : smooth;
mid1Res     = nentry("mid1:res",      0,  -25,  25, 10) : smooth;
mid2Freq    = nentry("mid2:freq",     0,    0,  50, 10) : smooth;
mid2Res     = nentry("mid2:res",      0,  -25,  25, 10) : smooth;
mid3Freq    = nentry("mid3:freq",     0,    0,  50, 10) : smooth;
mid3Res     = nentry("mid3:res",      0,  -25,  25, 10) : smooth;
highFreq    = nentry("high:freq",    50,    0,  50, 10) : smooth;
highRes     = nentry("high:res",      0,    0,  50, 10) : smooth;
attack      = nentry("attack",        0,    0,  50, 10) : smooth;
decay       = nentry("decay",         0,    0,  50, 10) : smooth;
sustain     = nentry("sustain",      50,    0,  50, 10) : smooth;
release     = nentry("release",       1,    0,  50, 10) : smooth;
volume      = nentry("volume",       25,    0,  50, 10) : smooth;
pan         = nentry("pan",           0,  -25,  25, 10) : smooth;

process = sound : adsr :> eq : mix;

sound = sample, synth1, synth2 with {
	sample = sp.stereoize(*(sampleLevel/50));
	synth1 = frequency(synth1Pitch) : oscillator(synth1Type) : *(synth1Level/50) <: _, _;
	synth2 = frequency(synth2Pitch) : oscillator(synth2Type) : *(synth2Level/50) <: _, _;
	frequency = /(10) : +(note) : ba.midikey2hz;
	oscillator = ba.selectmulti(1, (os.oscsin, os.triangle, os.sawtooth, os.square, (no.noise, !)));
};

adsr = sp.stereoize(*(sampleEnvelope)), sp.stereoize(*(envelope)), sp.stereoize(*(envelope)) with {
	sampleEnvelope = ba.if(adsrSample, envelope, 1);
	envelope = en.adsr(attack/20, decay/20, sustain/50, release/20, gate);
};

eq = sp.stereoize(low : high) with {
	low = ba.bypass1(lowFreq == 0, wa.highpass2(ba.midikey2hz(lowFreq*2 + 10), lowRes, 0));
	high = ba.bypass1(highFreq == 50, wa.lowpass2(ba.midikey2hz(highFreq*2), highRes, 0));
};

mix(inputL, inputR) = panned : sp.stereoize(*(volume/25)) with {
	panned = ba.select2stereo(panAmount > 0, toLeftL, toLeftR, toRightL, toRightR);
	panAmount = pan/25;
	toLeftL = inputL + inputR*abs(panAmount);
	toLeftR = inputR*(1+panAmount);
	toRightL = inputL*(1-panAmount);
	toRightR = inputR + inputL*panAmount;
};

smooth = si.polySmooth(trigger, amount, 1) with {
	trigger = gate : ba.peakhold(1);
	amount = 1 - 44.1 / ma.SR;
};
