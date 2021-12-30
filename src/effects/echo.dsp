import("stdfaust.lib");

gain = nentry("echoGain", 25,  0, 50, 10) : si.smoo;
x    = nentry("echoX",    25, -1, 50, 10) : si.smoo;
y    = nentry("echoY",    25,  0, 50, 10);

process = sp.stereoize(ba.selectmulti(1, (echo, reverseEcho), x == -1) : *(gain/10)) with {
	echo(s) = s : ef.echo(3, y/25, x/51) - s;
	reverseEcho = ef.reverseEchoN(1, 2^(12 + int(y/5)));
};
