import("stdfaust.lib");

scaled = library("scaled.lib");

gate = button("gate");
note = button("note");

duckRelease = button("duckRelease") : smooth;

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
lowFreq      = nentry("lowFreq",      0,  -25,  25, 10) : smooth;
lowRes       = nentry("lowRes",       0,  -25,  25, 10) : smooth;
midFreq      = nentry("midFreq",      0,  -25,  22, 10) : smooth;
midRes       = nentry("midRes",       0,  -25,  25, 10) : smooth;
highFreq     = nentry("highFreq",     0,  -25,  25, 10) : smooth;
highRes      = nentry("highRes",      0,  -25,  25, 10) : smooth;
attack       = nentry("attack",       0,    0,  50, 10) : smooth;
decay        = nentry("decay",        0,    0,  50, 10) : smooth;
sustain      = nentry("sustain",     50,    0,  50, 10) : smooth;
release      = nentry("release",      0,    0,  50, 10); // used in `smooth`
cutoff       = nentry("cutoff",       0,    0,  50, 10) : smooth;
pan          = nentry("pan",          0,  -25,  25, 10) : smooth;
main         = nentry("main",        50,    0,  50, 10) : smooth;
reverb       = nentry("reverb",       0,    0,  50, 10) : smooth;
echo         = nentry("echo",         0,    0,  50, 10) : smooth;
drive        = nentry("drive",        0,    0,  50, 10) : smooth;
toDuck       = nentry("toDuck",       0,    0,  50, 10) : smooth;
duckBy       = nentry("duckBy",       0,    0,  50, 10) : smooth;

process(prevL, prevR) = sound :> eq : pan_ : drive_ <: sends with {
	sends = send(toDuck), (duck_(prevL, prevR) <: send(main), send(echo), send(reverb));
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
	low = fi.low_shelf(scaled.filterGain(lowRes), scaled.filterFreq(300, lowFreq));
	mid = fi.peak_eq_cq(scaled.filterGain(midRes), scaled.filterFreq(1000, midFreq), 1);
	high = fi.high_shelf(scaled.filterGain(highRes), scaled.filterFreq(3000, highFreq - (cutoff/50 * envelope*25)));
};

pan_ = sp.panner(max(pan/25, 0)), sp.panner(1 + min(pan/25, 0)) :> _, _;

drive_ = sp.stereoize(ef.cubicnl_nodc(drive/50, .5));

// volume-ducking sidechain based on the previous `duck_mix`
duck_(prevL, prevR) = duck(prevL), duck(prevR) with {
	duck(prev) = *(1 - an.amp_follower_ar(0, scaled.time(duckRelease), min(1, prev*duckBy/25)));
};

send(amount) = sp.stereoize(*(amount/50));
envelope = en.adsr(scaled.time(attack), scaled.time(decay), sustain/50, scaled.time(release : smooth), gate);

smooth = si.polySmooth(trigger, amount, 1) with {
	trigger = en.ar(0, scaled.time(release), gate) : ma.signum;
	amount = 1 - 44.1/ma.SR; // https://github.com/grame-cncm/faustlibraries/blob/b54a01fa5ef0ac1f4939f78a88d318f1db85cc0a/signals.lib#L116
};
