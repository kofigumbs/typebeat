window.bindings = () => ({
  q: {
    name: "Sequence",
    icon: "grid",
    keyMap: {
      w: toggle("step:0", "Step"), e: toggle("step:1"),  r: toggle("step:2"),  t: toggle("step:3"),
      y: toggle("step:4"),         u: toggle("step:5"),  i: toggle("step:6"),  o: toggle("step:7"),
      s: toggle("step:8"),         d: toggle("step:9"),  f: toggle("step:10"), g: toggle("step:11"),
      h: toggle("step:12"),        j: toggle("step:13"), k: toggle("step:14"), l: toggle("step:15"),
      p: toggle("play", "▶", noActiveValue), ";": toggle("record", "●", noActiveValue), "/": custom.tempo,
      ...bottom8("length", "Len"),
    },
  },
  w: {
    name: "Track type",
    icon: "audio-device",
    keyMap: {
      ...right15("type", [ "Kit", "Mono", "Poly", "Arp", "Chrd", "Mic", "Mic●", "Mic▶" ]),
    },
  },
  e: {
    name: "Sample pack",
    icon: "notification",
    keyMap: {
      ...right15("sounds", [
        "808", "909", "DMX", "DNB", "Drk", "Deep",  "Tech",
        "Mod", "Gab", "Berg", "Vrm", "Cmd", "DMG", "FX4", "Snth",
      ]),
    },
  },
  r: {
    name: "Oct",
    icon: "chevrons-vertical",
    keyMap: {
      ...right15("octave", range(0, 8)),
    },
  },
  t: noBinding,
  a: {
    name: "Mix",
    icon: "chart-multiple",
    keyMap: {
      s: toggle("mute:0", "Mute"), d: toggle("mute:1"), f: toggle("mute:2"), g: toggle("mute:3"),
      h: toggle("mute:4"),         j: toggle("mute:5"), k: toggle("mute:6"), l: toggle("mute:7"),
      ...bottom8("track", "Trck"),
    },
  },
  s: {
    name: "Volume",
    icon: "arrows-vertical",
    keyMap: {
      ...right15("velocity", range(0, 14)),
    },
  },
  d: {
    name: "Pan",
    icon: "arrows-horizontal",
    keyMap: {
      ...right15("pan", range(-7, 7)),
    },
  },
  f: noBinding,
  g: noBinding,
  z: {
    name: "Key",
    icon: "invert",
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
    name: "Delay",
    icon: "section-copy",
    keyMap: {
      ...right15("delay", range(0, 14)),
    },
  },
  c: {
    name: "Reverb",
    icon: "missed-call",
    keyMap: {
      ...right15("reverb", range(0, 14)),
    },
  },
  v: noBinding,
  b: noBinding,
  [noModifier]: {
    name: "Note",
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
