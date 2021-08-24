import("stdfaust.lib");

gain  = nentry("driveGain",  25, 0, 50, 10) : si.smoo;
feed  = nentry("driveFeed",  25, 0, 50, 10) : si.smoo;
space = nentry("driveSpace", 15, 0, 50, 10) : si.smoo;

process = sp.stereoize(*(gain/50) : ef.cubicnl_nodc(feed/50, space/50));
