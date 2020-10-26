The macOS GarageBand/Logic samples are royalty-free, but can't be packaged and/or distributed.
They're all located in these two locations:

 - `/Library/Application\ Support/Logic/*Samples`
 - `/Library/Application\ Support/GarageBand/Instrument\ Library/Sampler`

I think SOUL can load any sample in one of the `.{aif,caf,wav}` formats, so I plan to write a script to generate the `.soulpatch` file using samples from these paths.
