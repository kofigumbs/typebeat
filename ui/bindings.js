window.bindings = () => ({
  q: [ "Seq", {
    ...top16("step", "Stp"),
    ...bottom8("length", "Len"),
  }],
  w: [ "Typ", {
    ...right15("type", [ "Kit", "Mon", "Pol", "Arp", "Chr" ]),
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
  a: [ "Trk", {
    // TODO ...middle8("mute", "Mut"),
    ...bottom8("track", "Sel"),
    "p": trig("play", "▶"), ";": trig("arm", "●"), // TODO "/": custom.tempo,
  }],
  s: [ "Vel", {
    // TODO ...right15("velocity", range(1, 15)),
  }],
  d: [ "Pan", {
    // TODO ...right15("pan", range(-7, 7)),
  }],
  f: [ "Flt", {
    // TODO ...right15("filter", range(-7, 7)),
  }],
  g: [ "Res", {
    // TODO ...right15("filter", range(1, 15)),
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
    // TODO ...right15("reverb", range(1, 15)),
  }],
  c: [ "Dly", {
    // TODO ...right15("delay", range(1, 15)),
  }],
  v: [ "", {
  }],
  b: [ "", {
  }],
  [noModifier]: [ "", {
    ...right15("key", Array(15).fill("~")),
  }],
});
