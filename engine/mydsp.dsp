import("stdfaust.lib");

noteCount = 15;
notes = par(i, noteCount, button("note:%i"));
playing = button("play") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("arm") : ba.toggle : hbargraph("armed", 0, 1);
trackType = nentry("setTrackType", 0, 0, 1, 1) : hbargraph("trackType", 0, 1);
instrument = nentry("setInstrument", 0, 0, 13, 1) : hbargraph("instrument", 0, 13);
key = nentry("setKey", 0, 0, 11, 1) : hbargraph("key", 0, 11);
octave = nentry("setOctave", 5, 0, 8, 1) : hbargraph("octave", 0, 8);
scale = nentry("setScale", 0, 0, 11, 1) : hbargraph("scale", 0, 11);
beat = hbargraph("beat", 0, 15);

// ```
//             ┌──────────────────────────────┬────────────────┬───────┬──────────
// ────────────┘                              ╵                ╵       ╵
// ^ init: 0   ^ set to 1 on first trig (b)   ^ negative pulse on subsequent trigs
// ```
trigger(b) = flip(ba.impulsify(b)) * ba.peakhold(1, b);
flip = xor(1); // `0` to `1` and vice versa

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

note = octave*12 + key + (scaleOffsets, int(_ + scale*15) : rdtable);
playInterpolated(targetNote, low, high, sound, channels, partOffset, frame) =
	slow, fast : par(i, 2, partOffset + useHigh, _ : sound) :> par(i, channels, it.interpolate_linear(interpolatedFrame - slow))
with {
	useHigh = abs(low - frame) > abs(high - frame);
	interpolatedFrame = frame * ba.semi2ratio(targetNote) / ba.semi2ratio(ba.if(useHigh, high, low));
	slow = floor(interpolatedFrame);
	fast = ceil(interpolatedFrame);
};

bpm = 180;
framesSince(hold) = (hold*_)~+(1) : _-1;
// thNote = int(framesSince(playing) / ba.tempo(bpm) * _ / 4);
// beatClock = 4:thNote : _ % 16 : beat;
// sequenceNotes = 8:thNote : rwtable(n, s, w, _, r) : *(playing);

playSample(sound, channels, part, frame) = min(part, 255), frame : sound : \(length, rate).(
	par(i, channels, _ * (frame < length))
);

enfer = playSample(soundfile("enfer", 2), 2);
enferKit(k, b) = ba.if(instrument == 13, k*18 + 15, k + instrument*18), framesSince(trigger(b)) : enfer;
enferSynth(k, b) = playInterpolated(note(k), 36, 48, enfer, 2, instrument*18 + 16, framesSince(trigger(b)));
enferNote(k, b) = enferKit(k, b), enferSynth(k, b) : ba.select2stereo(trackType);
liveNotes = notes : par(k, noteCount, _ : enferNote(k));

process = liveNotes :> _, _;
