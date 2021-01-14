import("stdfaust.lib");

bi = sp.stereoize;

stereoPan(amount, inputL, inputR) = ba.select2stereo(amount > 0,
	inputL + inputR*abs(amount),  inputR*(1+amount),
	inputL*(1-amount),            inputR + inputL*amount
);

effect(controls) = stereoPan(pan) : bi(*(velocity)) <: bi(_), bi(*(reverb)) with {
	velocity  = controlValue(0,  0, 1);
	pan       = controlValue(1, -1, 1);
	filter    = controlValue(2, -1, 1);
	resonance = controlValue(3,  0, 1);
	reverb    = controlValue(4,  0, 1);
	delay     = controlValue(5,  0, 1);
	controlValue(n, low, high) = ((controls >> (n*4)) & 15) / 14, low, high : it.interpolate_cosine;
};

freeverb_demo = _,_ <: (*(wet)*fixedgain,*(wet)*fixedgain :
	re.stereo_freeverb(combfeed, allpassfeed, damping, spatSpread)),
	*(1-wet), *(1-wet) :> _,_
with{
	scaleroom   = 0.28;
	offsetroom  = 0.7;
	allpassfeed = 0.5;
	scaledamp   = 0.4;
	fixedgain   = 0.1;
	origSR = 44100;

	wet = 1;
	damping = (.5)*scaledamp*origSR/ma.SR;
	combfeed = (.5)*scaleroom*origSR/ma.SR + offsetroom;
	spatSpread = (.5)*46*ma.SR/origSR : int;
};

voice(inputL, inputR, controls) = inputL, inputR : effect(controls);
process = par(i, 8*15, voice) :> bi(_), freeverb_demo :> bi(_);
