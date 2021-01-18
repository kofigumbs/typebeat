window.bindings = () => ({
  q: {
    name: "sequence",
    icon: "grid",
    keyMap: {
      w: toggle("step:0", "Step"), e: toggle("step:1"),  r: toggle("step:2"),  t: toggle("step:3"),
      y: toggle("step:4"),         u: toggle("step:5"),  i: toggle("step:6"),  o: toggle("step:7"),
      s: toggle("step:8"),         d: toggle("step:9"),  f: toggle("step:10"), g: toggle("step:11"),
      h: toggle("step:12"),        j: toggle("step:13"), k: toggle("step:14"), l: toggle("step:15"),
      p: toggle("play", "▶", noActiveValue),
      ";": toggle("record", "●", noActiveValue),
      "/": toggle("clear", "Clr", noActiveValue),
      ...bottom8("length", "Len"),
    },
  },
  w: {
    name: "source",
    icon: "mic",
    keyMap: {
      o: set("polyphonic", "Mono", 0), p: set("polyphonic", "Poly", 1),
      ...right15("source", [ "Kit", "Note", "In", "In ●", "In ▶" ]),
    },
  },
  e: {
    name: "sounds",
    icon: "bell",
    keyMap: {
      ...right15("sounds", [
        "808", "909", "DMX", "DnB", "Dark", "Deep",  "Tech",
        "Mod", "Gab", "Berg", "Verm", "Cmdr", "DMG", "FX4", "Snth",
      ]),
    },
  },
  r: {
    name: "octave",
    icon: "chevrons-up",
    keyMap: {
      ...right15("octave", range(0, 8)),
    },
  },
  t: noBinding,
  a: {
    name: "mix",
    icon: "sliders",
    keyMap: {
      s: toggle("mute:0", "Mute"), d: toggle("mute:1"), f: toggle("mute:2"), g: toggle("mute:3"),
      h: toggle("mute:4"),         j: toggle("mute:5"), k: toggle("mute:6"), l: toggle("mute:7"),
      p: custom.tempo,
      ...bottom8("track", "Trck"),
    },
  },
  s: {
    name: "volume",
    icon: "volume-1",
    keyMap: {
      ...right15("volume", range(0, 14)),
    },
  },
  d: {
    name: "pan",
    icon: "radio",
    keyMap: {
      ...right15("pan", range(-7, 7)),
    },
  },
  f: noBinding,
  g: noBinding,
  z: {
    name: "key",
    icon: "key",
    keyMap: {
      ...left12("root", [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
      ]),
      ...right12("scale", [
        "Maj", "Min",
        "Dor", "Phr", "Lyd", "Mix", "Loc", "HMi", "HMa", "MMi", "MMD", "MMa"
      ]),
    },
  },
  x: {
    name: "delay",
    icon: "copy",
    keyMap: {
      ...right15("delay", range(0, 14)),
    },
  },
  c: {
    name: "reverb",
    icon: "maximize",
    keyMap: {
      ...right15("reverb", range(0, 14)),
    },
  },
  v: noBinding,
  b: noBinding,
  [noModifier]: {
    name: "＼＿ﾍ(◕‿◕ )",
    icon: "",
    keyMap: {
      "y": toggle("key:10"), "u": toggle("key:11"), "i": toggle("key:12"), "o": toggle("key:13"), "p": toggle("key:14"),
      "h": toggle("key:5"),  "j": toggle("key:6"),  "k": toggle("key:7"),  "l": toggle("key:8"),  ";": toggle("key:9"),
      "n": toggle("key:0"),  "m": toggle("key:1"),  ",": toggle("key:2"),  ".": toggle("key:3"),  "/": toggle("key:4"),
    },
  },
  n: {
    ...noBinding, icon: "music"
  },
});
