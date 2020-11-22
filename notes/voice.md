This module takes advantage of the Faust ffi to sneak in some global state.
Faust does include the notion of polyphony, but the API is MIDI oriented, and thus seem inconvenient for Groovebox: https://github.com/grame-cncm/faust/blob/master-dev/architecture/faust/dsp/poly-dsp.h

Instead, we sort the possible voices (the 15 keys) by most recently triggered, and take the first 5.
We use the global arrays so that we only have to create and sort the array once per track.
This is safe and consistent because we always call `voiceKey` before we call `voicePosition`.
