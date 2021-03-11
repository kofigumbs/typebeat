import("stdfaust.lib");

process = par(i, 15, tgroup("%i", processVoice)) :> sp.stereoize(_);

processVoice = pan(panAmount) : sp.stereoize(*(volumeAmount)) with {
	volumeAmount = nentry("volume", 25, 0, 50, 1) : /(50) : *(2) : si.smoo;
	panAmount    = nentry("pan", 25, 0, 50, 1) : /(25) : -(1) : si.smoo;
};

pan(amount, inputL, inputR) = ba.select2stereo(amount > 0,
	inputL + inputR*abs(amount),  inputR*(1+amount),
	inputL*(1-amount),            inputR + inputL*amount
);

delay = sp.stereoize(ef.echo(3, .3, .8));

reverb = sp.stereoize(*(fixedgain)) : re.stereo_freeverb(combfeed, allpassfeed, damping, spatSpread) with {
	scaleroom   = 0.28;
	offsetroom  = 0.7;
	allpassfeed = 0.5;
	scaledamp   = 0.4;
	fixedgain   = 0.1;
	damping = (.5)*scaledamp;
	combfeed = (.5)*scaleroom + offsetroom;
	spatSpread = (.5)*46 : int;
};
