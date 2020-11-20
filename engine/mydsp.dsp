import("stdfaust.lib");

/*
 * ui
 */

keyCount = 15;
keys = par(i, keyCount, button("key:%i"));

beatCount = 16;
stepToggles = par(i, beatCount, button("toggleStep:%i"));

playing = button("play") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("arm") : ba.toggle : hbargraph("armed", 0, 1);
trackType = nentry("setTrackType", 0, 0, 1, 1) : hbargraph("trackType", 0, 1);
instrument = nentry("setInstrument", 0, 0, 13, 1) : hbargraph("instrument", 0, 13);
root = nentry("setRoot", 0, 0, 11, 1) : hbargraph("root", 0, 11);
octave = nentry("setOctave", 4, 0, 8, 1) : hbargraph("octave", 0, 8);
scale = nentry("setScale", 0, 0, 11, 1) : hbargraph("scale", 0, 11);
beat = hbargraph("beat", 0, beatCount - 1);

lastKey = keys : par(i, keyCount, ba.impulsify : *(i+1)) : parallelOp(max, keyCount) : table : hbargraph("lastKey", 0, keyCount - 1) with {
	table(key) = rwtable(2, 0, key != 0, key - 1, always(1));
};


/*
 * math
 */

// https://github.com/grame-cncm/faust/issues/423
always = _, int(hslider("~nothing", 0, 0, 0, 0)) : max;

// https://github.com/grame-cncm/faustlibraries/blob/1e7bf622df13b0a130bfd49e857392f697044f06/basics.lib#L1636-L1638
parallelOp(op,1) = _;
parallelOp(op,2) = op;
parallelOp(op,n) = op(parallelOp(op,n-1));

// ```
//             ┌──────────────────────────────┬────────────────┬───────┬──────────
// ────────────┘                              ╵                ╵       ╵
// ^ init: 0   ^ set to 1 on first trig (b)   ^ negative pulse on subsequent trigs
// ```
holdPulse(b) = flip(ba.impulsify(b)) * ba.peakhold(1, b);
flip = xor(1); // `0` to `1` and vice versa
clamp(low, high) = min(high, max(low, _));


/*
 * time
 */

framesSince(hold) = (hold*_) ~ +(1) : _-1;
setOrToggle(set, toggle) = _ ~ \(prev).(set | select2(toggle, prev, flip(prev)));


/*
 * tones
 */

scaleOffsets = waveform {
		-12, -10, -8, -7, -5, -3, -1, 0, 2, 4, 5, 7, 9, 11, 12,
		-12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12,
		-12, -10, -9, -7, -5, -3, -2, 0, 2, 3, 5, 7, 9, 10, 12,
		-12, -11, -9, -7, -5, -4, -2, 0, 1, 3, 5, 7, 8, 10, 12,
		-12, -10, -8, -6, -5, -3, -1, 0, 2, 4, 6, 7, 9, 11, 12,
		-12, -10, -8, -7, -5, -3, -2, 0, 2, 4, 5, 7, 9, 10, 12,
		-12, -11, -9, -7, -6, -4, -2, 0, 1, 3, 5, 6, 8, 10, 12,
		-12, -10, -9, -7, -5, -4, -1, 0, 2, 3, 5, 7, 8, 11, 12,
		-12, -10, -8, -7, -5, -4, -1, 0, 2, 4, 5, 7, 8, 11, 12,
		-12, -10, -9, -7, -5, -3, -1, 0, 2, 3, 5, 7, 9, 11, 12,
		-12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12,
		-12, -10, -8, -7, -5, -4, -2, 0, 2, 4, 5, 7, 8, 10, 12
};
note(key) = octave*12 + root + (scaleOffsets, int(key + scale*15) : rdtable);

playInterpolated(sound, channels, partOffset, frame, targetNote, low, high) =
	slow, fast : par(i, 2, partOffset + useHigh, _ : sound) :> par(i, channels, it.interpolate_linear(interpolatedFrame - slow))
with {
	useHigh = abs(low - frame) > abs(high - frame);
	interpolatedFrame = frame * ba.semi2ratio(targetNote) / ba.semi2ratio(ba.if(useHigh, high, low));
	slow = floor(interpolatedFrame);
	fast = ceil(interpolatedFrame);
};

playSample(sound, channels, part, frame) = part, frame : sound : untilEnd with {
	untilEnd(length, rate) = par(i, channels, *(frame < length));
};

enfer = clamp(0, 255, _), _ : playSample(soundfile("enfer", 2), 2);
enferKit(key, trig) = ba.if(instrument == 13, key*18 + 15, key + instrument*18), framesSince(holdPulse(trig)) : enfer;
enferSynth(key, trig) = instrument*18 + 16, framesSince(holdPulse(trig)), note(key), 36, 48 : playInterpolated(enfer, 2);
enferKey(key, trig) = enferKit(key, trig), enferSynth(key, trig) : ba.select2stereo(trackType);


/*
 * sequence
 */

bpm = 180;
stepsPerBeat = 2;
stepsInSequence = beatCount * stepsPerBeat;
stepPosition(subdivision) = int(framesSince(playing) / ba.tempo(bpm) * subdivision);
stepMod = stepPosition(stepsPerBeat) % stepsInSequence;
clock = stepMod, beat(int(stepMod/stepsPerBeat)) : max; // same as `stepMod`, but passes through and updates `beat`

stepTrigger(keyI, keyB, stepI, stepB) = sequenceTrigger | liveTrigger with {
	liveTrigger = keyB : ba.impulsify : *(stepI == clock) : ba.if(playing & armed & (clock != quantClock), 0);
	sequenceTrigger = recorded, toggled : setOrToggle : *(stepI == clock) : *(playing) : ba.impulsify;
	toggled = stepB : ba.impulsify : *(lastKey == keyI);
	recorded = keyB : ba.impulsify * armed * playing * (stepI == quantClock);
	quantClock = (stepPosition(2*stepsPerBeat) + 1) / 2 : int : %(stepsInSequence);
};
beatTrigger(keyI, keyB, beatI, beatB) = sum(i, stepsPerBeat, keyI, keyB, i + beatI*stepsPerBeat, (i==0)*beatB : stepTrigger);
voiceTrigger(keyI, keyB) = stepToggles : sum(i, beatCount, keyI, keyB, i, _ : beatTrigger);


/*
 * output
 */

process = keys : par(i, keyCount, voiceTrigger(i) : enferKey(i)) :> _, _;
