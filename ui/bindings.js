window.bindings = () => ({
  q: [ "Seq", {
    w: toggle("step:0", "Stp"), e: toggle("step:1"),  r: toggle("step:2"),  t: toggle("step:3"),
    y: toggle("step:4"),        u: toggle("step:5"),  i: toggle("step:6"),  o: toggle("step:7"),
    s: toggle("step:8"),        d: toggle("step:9"),  f: toggle("step:10"), g: toggle("step:11"),
    h: toggle("step:12"),       j: toggle("step:13"), k: toggle("step:14"), l: toggle("step:15"),
    ...bottom8("length", "Len"),
  }],
  w: [ "Typ", {
    ...right15("type", [ "Kit", "Mon", "Pol", "Arp", "Chr", "In", "In●", "In▶" ]),
  }],
  e: [ "Snd", {
    ...right15("sounds", [
      "808", "909", "DMX", "DNB", "Drk", "Dp",  "Tch",
      "Mod", "Gab", "Brg", "Vrm", "Cmd", "DMG", "FX4", "Syn",
    ]),
  }],
  r: [ "Oct", {
    ...right15("octave", range(0, 8)),
  }],
  t: [ "", {
  }],
  a: [ "Mix", {
    s: toggle("mute:0", "Mut"), d: toggle("mute:1"), f: toggle("mute:2"), g: toggle("mute:3"),
    h: toggle("mute:4"),        j: toggle("mute:5"), k: toggle("mute:6"), l: toggle("mute:7"),
    ...bottom8("track", "Trk"),
    p: toggle("play", "▶", noActiveValue), ";": toggle("arm", "●", noActiveValue), "/": custom.bpm,
  }],
  s: [ "Vel", {
    ...right15("velocity", range(0, 14)),
  }],
  d: [ "Pan", {
    ...right15("pan", range(-7, 7)),
  }],
  f: [ "Flt", {
    ...right15("filter", range(-7, 7)),
  }],
  g: [ "Res", {
    ...right15("resonance", range(0, 14)),
  }],
  z: [ "Key", {
    ...left12("root", [
      "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ]),
    ...right12("scale", [
      "Maj", "Min",
      "Dor", "Phr", "Lyd", "Mix", "Loc", "HMi", "HMa", "MMi", "MMD", "MMa"
    ]),
  }],
  x: [ "Rev", {
    ...right15("reverb", range(0, 14)),
  }],
  c: [ "Dly", {
    ...right15("delay", range(0, 14)),
  }],
  v: [ "", {
  }],
  b: [ "", {
  }],
  [noModifier]: [ "", {
    "y": toggle("key:10", "~"), "u": toggle("key:11", "~"), "i": toggle("key:12", "~"), "o": toggle("key:13", "~"), "p": toggle("key:14", "~"),
    "h": toggle("key:5", "~"),  "j": toggle("key:6", "~"),  "k": toggle("key:7", "~"),  "l": toggle("key:8", "~"),  ";": toggle("key:9", "~"),
    "n": toggle("key:0", "~"),  "m": toggle("key:1", "~"),  ",": toggle("key:2", "~"),  ".": toggle("key:3", "~"),  "/": toggle("key:4", "~"),
  }],
});
