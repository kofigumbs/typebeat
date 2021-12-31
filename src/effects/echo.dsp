import("stdfaust.lib");

gain   = nentry("echoGain",   25,  0, 50, 10) : si.smoo;
length = nentry("echoLength", 25, -1, 50, 10) : si.smoo;
feed   = nentry("echoFeed",   25,  0, 50, 10);

process = sp.stereoize(ba.selectmulti(1, (echo, reverseEcho), length == -1) : *(gain/10)) with {
	echo(s) = s : ef.echo(3, feed/25, length/51) - s;
	reverseEcho = ef.reverseEchoN(1, 2^(12 + int(feed/5)));
};
