import("stdfaust.lib");

gain  = nentry("echoGain",  25,  0, 50, 10) : si.smoo;
feed  = nentry("echoFeed",  25, -1, 50, 10);
space = nentry("echoSpace", 25,  0, 50, 10);

process = sp.stereoize(ba.selectmulti(1, (echo, reverseEcho), feed == -1) : *(gain/10)) with {
	echo = ef.echo(2, space/25, feed/51);
	reverseEcho = ef.reverseEchoN(1, 2^(12 + int(space/5)));
};
