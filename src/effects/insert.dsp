import("stdfaust.lib");

gate = nentry("gate",  0, 0,   1, 0);
thru = nentry("thru",  0, 0,   1, 0);
note = nentry("note", 69, 0, 127, 0);

synth1Type = nentry("synth 1 type", 0, 0, 4, 1);
synth2Type = nentry("synth 2 type", 0, 0, 4, 1);
synth3Type = nentry("synth 3 type", 0, 0, 4, 1);
holdSample = nentry("holdSample",   0, 0, 1, 0);

sampleLevel  = nentry("sample level",  25,    0,  50, 10) : smooth;
sampleDetune = nentry("sample detune",  0, -120, 120, 10) : smooth;
synth1Level  = nentry("synth 1 level",  0,    0,  50, 10) : smooth;
synth1Detune = nentry("synth 1 detune", 0, -120, 120, 10) : smooth;
synth2Level  = nentry("synth 2 level",  0,    0,  50, 10) : smooth;
synth2Detune = nentry("synth 2 detune", 0, -120, 120, 10) : smooth;
synth3Level  = nentry("synth 3 level",  0,    0,  50, 10) : smooth;
synth3Detune = nentry("synth 3 detune", 0, -120, 120, 10) : smooth;
lowFreq      = nentry("low freq",       0,    0,  50, 10) : smooth;
lowRes       = nentry("low res",        0,    0,  50, 10) : smooth;
band1Freq    = nentry("band 1 freq",    0,  -25,  25, 10) : smooth;
band1Res     = nentry("band 1 res",     0,  -25,  25, 10) : smooth;
band2Freq    = nentry("band 2 freq",    0,  -25,  25, 10) : smooth;
band2Res     = nentry("band 2 res",     0,  -25,  25, 10) : smooth;
band3Freq    = nentry("band 3 freq",    0,  -25,  25, 10) : smooth;
band3Res     = nentry("band 3 res",     0,  -25,  25, 10) : smooth;
highFreq     = nentry("high freq",     50,    0,  50, 10) : smooth;
highRes      = nentry("high res",       0,    0,  50, 10) : smooth;
attack       = nentry("attack",         0,    0,  50, 10) : smooth;
decay        = nentry("decay",          0,    0,  50, 10) : smooth;
sustain      = nentry("sustain",       50,    0,  50, 10) : smooth;
release      = nentry("release",        0,    0,  50, 10) : smooth;
cutoff       = nentry("cutoff",         0,    0,  50, 10) : smooth;
pan          = nentry("pan",            0,  -25,  25, 10) : smooth;
main         = nentry("main",          50,    0,  50, 10) : smooth;
reverb       = nentry("reverb",         0,    0,  50, 10) : smooth;
echo         = nentry("echo",           0,    0,  50, 10) : smooth;
drive        = nentry("drive",          0,    0,  50, 10) : smooth;

process = sound :> eq : panning <: send(main), send(reverb), send(echo), send(drive);

sound = sample, synth1, synth2, synth3 with {
	sample = sp.stereoize(sampleTranspose : *(sampleLevel/25 * ba.if(holdSample, envelope, 1)));
	sampleTranspose = ba.bypass_fade(1, sampleOffset == 0, ef.transpose(1000, 10, sampleOffset));
	sampleOffset = thru * (sampleDetune/10 + note - 69);
	synth1 = frequency(synth1Detune/10) : oscillator(synth1Type) : *(synth1Level/150 * envelope) <: _, _;
	synth2 = frequency(synth2Detune/10) : oscillator(synth2Type) : *(synth2Level/150 * envelope) <: _, _;
	synth3 = frequency(synth3Detune/10) : oscillator(synth3Type) : *(synth3Level/150 * envelope) <: _, _;
	frequency = _/10 + note : ba.midikey2hz;
	oscillator = ba.selectmulti(1, (os.oscsin, os.triangle, os.sawtooth/2, os.square/2, (no.noise/4, !)));
};

eq = sp.stereoize(low : band1 : band2 : band3 : high) with {
	low = ba.bypass_fade(1, lowFreq == 0, wa.highpass2(ba.midikey2hz(lowFreq*2 - 10), lowRes, 0));
	band1 = ba.bypass_fade(1, (band1Freq == 0) & (band1Res == 0), wa.peaking2(band1Freq/2 + 36, band1Res/2, 4, 0));
	band2 = ba.bypass_fade(1, (band2Freq == 0) & (band2Res == 0), wa.peaking2(band2Freq/2 + 60, band2Res/2, 4, 0));
	band3 = ba.bypass_fade(1, (band3Freq == 0) & (band3Res == 0), wa.peaking2(band3Freq/2 + 74, band3Res/2, 4, 0));
	high = ba.bypass_fade(1, highCut == 50, wa.lowpass2(ba.midikey2hz(highCut + 60), highRes, 0));
	highCut = highFreq - it.interpolate_linear(cutoff/50, 0, envelope * 50);
};

panning(inputL, inputR) = ba.select2stereo(pan > 25, toLeftL, toLeftR, toRightL, toRightR) with {
	toLeftL = inputL + inputR*abs(pan/25);
	toLeftR = inputR*(1 + pan/25);
	toRightL = inputL*(1 - pan/25);
	toRightR = inputR + inputL*pan/25;
};

send(amount) = sp.stereoize(*(amount/50));
envelope = en.adsr(attack/50, decay/50, sustain/50, release/50, gate);

smooth = si.polySmooth(trigger, amount, 1) with {
	trigger = gate : ba.peakhold(1);
	amount = 1 - 44.1/ma.SR; // https://github.com/grame-cncm/faustlibraries/blob/b54a01fa5ef0ac1f4939f78a88d318f1db85cc0a/signals.lib#L116
};
