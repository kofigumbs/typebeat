import("stdfaust.lib");

process = par(i, 15, processVoice(i)) :> sp.stereoize(_);

processVoice(voice) = pan(panAmount) : sp.stereoize(*(volumeAmount)) with {
	volumeAmount = getAmount(mix, voice, 0, 0, 2);
	panAmount    = getAmount(mix, voice, 1, -1, 1);
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

getAmount(f, voice, id, low, high) = f(voice, id, ba.time) / 50, low, high : it.interpolate_cosine : si.smoo;

eq = ffunction(int getEq (int, int, int), "foreign.h", "");
envelope = ffunction(int getEnvelope (int, int, int), "foreign.h", "");
effect = ffunction(int getEffect (int, int, int), "foreign.h", "");
mix = ffunction(int getMix (int, int, int), "foreign.h", "");
