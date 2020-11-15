import("stdfaust.lib");

noteCount = 15;
notes = par(i, noteCount, button("note:%i"));
playing = button("play") : ba.toggle : hbargraph("playing", 0, 1);
armed = button("arm") : ba.toggle : hbargraph("armed", 0, 1);
trackType = nentry("setTrackType", 0, 0, 1, 1) : hbargraph("trackType", 0, 1);
instrument = nentry("setInstrument", 0, 0, 13, 1) : hbargraph("instrument", 0, 13);
key = nentry("setKey", 0, 0, 11, 1) : hbargraph("key", 0, 11);
octave = nentry("setOctave", 5, 0, 8, 1) : hbargraph("octave", 0, 8);
beat = hbargraph("beat", 0, 15);

// ```
//             ┌──────────────────────────────┬────────────────┬───────┬──────────
// ────────────┘                              ╵                ╵       ╵
// ^ init: 0   ^ set to 1 on first trig (b)   ^ negative pulse on subsequent trigs
// ```
trigger(b) = flip(ba.impulsify(b)) * ba.peakhold(1, b);

// `0` to `1` and vice versa
flip = xor(1);

note = _ + octave*12 + key; // TODO scale
readInterpolated(targetNote, soundPartOffset, low, high, i, sound) =
	slow, fast : par(i, 2, soundPartOffset + useHigh, _ : sound) :> par(i, 2, it.interpolate_linear(interpolatedI - slow))
with {
	useHigh = abs(low - i) > abs(high - i);
	interpolatedI = i * ba.semi2ratio(targetNote) / ba.semi2ratio(ba.if(useHigh, high, low));
	slow = floor(interpolatedI);
	fast = ceil(interpolatedI);
};

bpm = 180;
framesSince(hold) = (hold*_)~+(1) : _-1;
// thNote(n) = int(framesSince(playing) / ba.tempo(bpm) * n / 4);
// beatClock = 4:thNote : _ % 16 : beat;
// sequence_notes = 8:thNote : rwtable(n, s, w, _, r) : *(playing);

enfer = min(_, 255), _ : soundfile("enfer", 2) : !,!,_,_;
enferKit(k, b) = ba.if(instrument == 13, k*18 + 15, k + instrument*18), framesSince(trigger(b)) : enfer;
enferSynth(k, b) = readInterpolated(note(k), instrument*18 + 16, 36, 48, framesSince(trigger(b)), enfer);
enferNote(k, b) = enferKit(k, b), enferSynth(k, b) : ba.select2stereo(trackType);
liveNotes = notes : par(k, noteCount, _ : enferNote(k));

process = liveNotes :> _, _;
