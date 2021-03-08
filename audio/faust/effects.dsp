import("stdfaust.lib");

process = par(i, 15, processVoice(i)) :> sp.stereoize(_);

processVoice(voice) = pan(panAmount) : sp.stereoize(*(volumeAmount)) with {
	volumeAmount = getAmount(mix, voice, 0) : *(2) : si.smoo;
	panAmount    = getAmount(mix, voice, 5), -1, 1 : it.interpolate_linear : si.smoo;
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

getAmount(f, voice, id) = f(voice, id, ba.time) / 50;

eq = ffunction(int getEq (int, int, int), "foreign.h", "");
adsr = ffunction(int getAdsr (int, int, int), "foreign.h", "");
fx = ffunction(int getFx (int, int, int), "foreign.h", "");
mix = ffunction(int getMix (int, int, int), "foreign.h", "");
