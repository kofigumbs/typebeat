import("stdfaust.lib");

process = par(i, 8*15, voice) :> bi(_), sendEffects :> bi(_);
voice(inputL, inputR, controls) = inputL, inputR : insertEffects(controls);
sendEffects(dl, dr, rl, rr) = dl, dr : bi(ef.echo(3, .15, .9)) : +(rl), +(rr) : stereoReverb;

insertEffects(controls) = stereoPan(pan) : bi(*(velocity)) <: bi(_), bi(*(delay)), bi(*(reverb)) with {
	velocity  = controlValue(0,  0, 1);
	pan       = controlValue(1, -1, 1);
	filter    = controlValue(2, -1, 1);
	resonance = controlValue(3,  0, 1);
	delay     = controlValue(4,  0, 1);
	reverb    = controlValue(5,  0, 1);
	controlValue(n, low, high) = ((controls >> (n*4)) & 15) / 14, low, high : it.interpolate_cosine : si.smoo;
};

stereoPan(amount, inputL, inputR) = ba.select2stereo(amount > 0,
	inputL + inputR*abs(amount),  inputR*(1+amount),
	inputL*(1-amount),            inputR + inputL*amount
);

stereoReverb = bi(*(fixedgain)) : re.stereo_freeverb(combfeed, allpassfeed, damping, spatSpread) with {
	scaleroom   = 0.28;
	offsetroom  = 0.7;
	allpassfeed = 0.5;
	scaledamp   = 0.4;
	fixedgain   = 0.1;
	damping = (.5)*scaledamp;
	combfeed = (.5)*scaleroom + offsetroom;
	spatSpread = (.5)*46 : int;
};

bi = sp.stereoize;
