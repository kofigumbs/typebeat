import("stdfaust.lib");

level    = nentry("level",      1, 0, 1, .01);
dtime    = nentry("delay",    .04, 0, 5, .001);
feedback = nentry("feedback",   0, 0, 1, .001);
stereo   = nentry("stereo",     1, 0, 1, .001);

echo(dtime,level,feedback,stereo,x,y)
		= f(x,y) // the echo loop
		// mix
		: (\(u,v).(x+level*(d(u)+c(v)),
			   y+level*(d(v)+c(u))))
		// compensate for gain level
		: (/(1+level), /(1+level))
with {
	f = g ~ (*(feedback),*(feedback));
	g(u,v,x,y) = h(x+d(u)+c(v)), h(y+d(v)+c(u));
	h = de.fdelay(1<<18, ma.SR*dtime);
	c(x) = x*stereo;
	d(x) = x*(1-stereo);
};

process = echo(dtime,level,feedback,stereo);
