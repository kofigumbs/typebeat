import("stdfaust.lib");

gain  = nentry("drive gain",  25, 0, 50, 10) : si.smoo;
feed  = nentry("drive feed",  25, 0, 50, 10) : si.smoo;
space = nentry("drive space", 15, 0, 50, 10) : si.smoo;

process = sp.stereoize(*(gain/50) : ef.cubicnl_nodc(feed/50, space/50));
