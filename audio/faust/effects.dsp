import("stdfaust.lib");

process = par(i, 15, voiceEffects(i)) :> sp.stereoize(_);

voiceEffects(voice) = stereoPan(pan) : sp.stereoize(*(volume)) with {
	chorus  = parameter(0, 0, 1);
	distort = parameter(1, 0, 1);
	crush   = parameter(2, 0, 1);
	delay   = parameter(3, 0, 1);
	reverb  = parameter(4, 0, 1);
	volume  = parameter(5, 0, 1);
	pan     = parameter(6, -1, 1);
	parameter(id, low, high) = getParameter(voice, id, ba.time) / 50, low, high : it.interpolate_cosine : si.smoo;
};

stereoPan(amount, inputL, inputR) = ba.select2stereo(amount > 0,
	inputL + inputR*abs(amount),  inputR*(1+amount),
	inputL*(1-amount),            inputR + inputL*amount
);

stereoDelay = sp.stereoize(ef.echo(3, .3, .8));

stereoReverb = sp.stereoize(*(fixedgain)) : re.stereo_freeverb(combfeed, allpassfeed, damping, spatSpread) with {
	scaleroom   = 0.28;
	offsetroom  = 0.7;
	allpassfeed = 0.5;
	scaledamp   = 0.4;
	fixedgain   = 0.1;
	damping = (.5)*scaledamp;
	combfeed = (.5)*scaleroom + offsetroom;
	spatSpread = (.5)*46 : int;
};

getParameter = ffunction(int getParameter (int, int, int), "foreign.h", "");
