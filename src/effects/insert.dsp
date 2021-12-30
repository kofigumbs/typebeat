import("stdfaust.lib");

gate = button("gate");
note = button("note");

duckGain = button("duckGain");
duckX    = button("duckX");
duckY    = button("duckY");

sampleType = nentry("sampleType", 0, 0, 4, 1);
synth1Type = nentry("synth1Type", 0, 0, 4, 1);
synth2Type = nentry("synth2Type", 0, 0, 4, 1);
synth3Type = nentry("synth3Type", 0, 0, 4, 1);
holdSample = nentry("holdSample", 0, 0, 1, 0);

sampleLevel  = nentry("sampleLevel", 25,    0,  50, 10) : smooth;
sampleDetune = nentry("sampleDetune", 0, -120, 120, 10) : smooth;
synth1Level  = nentry("synth1Level",  0,    0,  50, 10) : smooth;
synth1Detune = nentry("synth1Detune", 0, -120, 120, 10) : smooth;
synth2Level  = nentry("synth2Level",  0,    0,  50, 10) : smooth;
synth2Detune = nentry("synth2Detune", 0, -120, 120, 10) : smooth;
synth3Level  = nentry("synth3Level",  0,    0,  50, 10) : smooth;
synth3Detune = nentry("synth3Detune", 0, -120, 120, 10) : smooth;
lowFreq      = nentry("lowFreq",      0,    0,  50, 10) : smooth;
lowRes       = nentry("lowRes",       0,  -50,   0, 10) : smooth;
midFreq      = nentry("midFreq",      0,  -25,  25, 10) : smooth;
midRes       = nentry("midRes",       0,  -25,  25, 10) : smooth;
highFreq     = nentry("highFreq",    50,    0,  50, 10) : smooth;
highRes      = nentry("highRes",      0,  -50,   0, 10) : smooth;
attack       = nentry("attack",       0,    0,  50, 10) : smooth;
decay        = nentry("decay",        0,    0,  50, 10) : smooth;
sustain      = nentry("sustain",     50,    0,  50, 10) : smooth;
release      = nentry("release",      0,    0,  50, 10);
cutoff       = nentry("cutoff",       0,    0,  50, 10) : smooth;
pan          = nentry("pan",          0,  -25,  25, 10) : smooth;
main         = nentry("main",        50,    0,  50, 10) : smooth;
reverb       = nentry("reverb",       0,    0,  50, 10) : smooth;
echo         = nentry("echo",         0,    0,  50, 10) : smooth;
toDuck       = nentry("toDuck",       0,    0,  50, 10) : smooth;
duckBy       = nentry("duckBy",       0,    0,  50, 10) : smooth;

process(prevL, prevR) = sound :> eq : panning <: sends with {
	sends = send(toDuck), (ducking(prevL, prevR) <: send(main), send(reverb), send(echo));
};

sound = sample, synth1, synth2, synth3 with {
	sample = sp.stereoize(sampleTranspose : *(sampleLevel/25 * ba.if(holdSample, envelope, 1)));
	sampleTranspose = ba.bypass_fade(1, sampleOffset == 0, ef.transpose(1000, 10, sampleOffset));
	sampleOffset = (sampleType == 1 | sampleType == 2) * (sampleDetune/10 + note - 69);
	synth1 = frequency(synth1Detune/10) : oscillator(synth1Type) : *(synth1Level/150 * envelope) <: _, _;
	synth2 = frequency(synth2Detune/10) : oscillator(synth2Type) : *(synth2Level/150 * envelope) <: _, _;
	synth3 = frequency(synth3Detune/10) : oscillator(synth3Type) : *(synth3Level/150 * envelope) <: _, _;
	frequency = _/10 + note : ba.midikey2hz;
	oscillator = ba.selectmulti(1, (os.oscsin, os.triangle, os.sawtooth/2, os.square/2, (no.noise/4, !)));
};

eq = sp.stereoize(low : mid : high) with {
	low = ba.bypass_fade(1, lowFreq == 0, wa.highpass2(ba.midikey2hz(lowFreq*2 - 10), lowRes, 0));
	mid = ba.bypass_fade(1, (midFreq == 0) & (midRes == 0), wa.peaking2(midFreq/2 + 60, midRes/2, 4, 0));
	high = ba.bypass_fade(1, highCut == 50, wa.lowpass2(ba.midikey2hz(highCut + 60), highRes, 0));
	highCut = highFreq - it.interpolate_linear(cutoff/50, 0, envelope * 50);
};

panning(inputL, inputR) = ba.select2stereo(pan > 25, toLeftL, toLeftR, toRightL, toRightR) with {
	toLeftL = inputL + inputR*abs(pan/25);
	toLeftR = inputR*(1 + pan/25);
	toRightL = inputL*(1 - pan/25);
	toRightR = inputR + inputL*pan/25;
};

ducking(prevL, prevR) = duck(prevL), duck(prevR) with {
	duck(prev) = *(1 - an.amp_follower_ar(time(duckX), time(duckY), min(1, prev*duckBy/(51 - duckGain))));
};

send(amount) = sp.stereoize(*(amount/50));
envelope = en.adsr(time(attack), time(decay), sustain/50, (release : smooth : time), gate);
time = _/50;

smooth = si.polySmooth(trigger, amount, 1) with {
	trigger = en.ar(0, time(release), gate) : ma.signum;
	amount = 1 - 44.1/ma.SR; // https://github.com/grame-cncm/faustlibraries/blob/b54a01fa5ef0ac1f4939f78a88d318f1db85cc0a/signals.lib#L116
};
