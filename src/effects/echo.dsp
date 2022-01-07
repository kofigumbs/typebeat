import("stdfaust.lib");

scaled = library("scaled.lib");

gain   = nentry("echoGain",   25,  0, 50, 10) : si.smoo;
length = nentry("echoLength", 25, -1, 50, 10) : si.smoo;
feed   = nentry("echoFeed",   25,  0, 50, 10);

process = sp.stereoize(ba.selectmulti(1, (echo, reverseEcho), length < 0) : *(gain/10)) with {
	echo(s) = s : ef.echo(6, scaled.time(length), feed/51) - s;
	reverseEcho = ef.reverseEchoN(1, pow(2, 12 + int(feed/5)));
};
