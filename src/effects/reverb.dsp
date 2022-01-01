import("stdfaust.lib");

gain = nentry("reverbGain", 25, 0, 50, 10) : si.smoo;
size = nentry("reverbSize", 25, 0, 50, 10);

process = sp.stereoize(*(gain/250)) : re.stereo_freeverb(size*.0056 + .7, .5, .2, 23);
