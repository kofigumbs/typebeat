import("stdfaust.lib");

gain   = nentry("echo:gain",   25,  0, 50, 10) : si.smoo;
feed   = nentry("echo:feed",   25, -1, 50, 10);
length = nentry("echo:length", 15,  0, 50, 10);

process = sp.stereoize(ba.selectmulti(1, (echo, reverseEcho), feed == -1) : *(gain/10)) with {
	echo = ef.echo(2, length/25, feed/51);
	reverseEcho = ef.reverseEchoN(1, 2^(12 + int(length/5)));
};
