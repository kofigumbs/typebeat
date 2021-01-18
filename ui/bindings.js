window.bindings = () => ({
  q: {
    name: "sequence",
    icon: "grid",
    keyMap: {
      "p": toggle("play", "▶", noActiveValue),
      ";": toggle("record", "●", noActiveValue),
      "/": toggle("clear", "×", noActiveValue),
      ...from("wertyuiosdfghjkl", (key, i) => toggle(`step:${i}`, title(i, "Step"))),
      ...from("xcvbnm,.", (key, i) => set("length", title(i, "Len"), i)),
    },
  },
  w: {
    name: "source",
    icon: "mic",
    keyMap: {
      ...from("op", (key, i) => set("polyphonic", ["Mono", "Poly"][i], i)),
      ...from("nm,./", (key, i) => set("source", ["Kit", "Note", "In", "In ●", "In ▶"][i], i)),
    },
  },
  e: {
    name: "sounds",
    icon: "bell",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => set("sounds", ["808", "909", "DMX", "DnB", "Dark", "Deep", "Tech", "Mod", "Gab", "Berg", "Verm", "Cmdr", "DMG", "FX4", "Snth"][i], i)),
    },
  },
  r: {
    name: "octave",
    icon: "chevrons-up",
    keyMap: {
      ...from("nm,./hjkl", (key, i) => set("octave", `${i}`, i)),
    },
  },
  t: noBinding,
  a: {
    name: "mix",
    icon: "sliders",
    keyMap: {
      "p": custom.tempo,
      ...from("sdfghjkl", (key, i) => toggle(`mute:${i}`, title(i, "Mute"))),
      ...from("xcvbnm,.", (key, i) => set("track", title(i, "Trck"), i)),
    },
  },
  s: {
    name: "volume",
    icon: "volume-1",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => set("volume", `${i}`, i)),
    },
  },
  d: {
    name: "pan",
    icon: "radio",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => set("pan", `${i-7}`, i)),
    },
  },
  f: noBinding,
  g: noBinding,
  z: {
    name: "key",
    icon: "key",
    keyMap: {
      ...from("xcvbsdfgwert", (key, i) => set("root", ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][i], i)),
      ...from("nm,.hjklyuio", (key, i) => set("scale", ["Maj", "Min", "Dor", "Phr", "Lyd", "Mix", "Loc", "HMi", "HMa", "MMi", "MMD", "MMa"][i], i)),
    },
  },
  x: {
    name: "delay",
    icon: "copy",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => set("delay", `${i}`, i)),
    },
  },
  c: {
    name: "reverb",
    icon: "maximize",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => set("reverb", `${i}`, i)),
    },
  },
  v: noBinding,
  b: noBinding,
  [noModifier]: {
    name: "＼＿ﾍ(◕‿◕ )",
    icon: "",
    keyMap: {
      ...from("nm,./hjkl;yuiop", (key, i) => toggle(`key:${i}`)),
    },
  },
  n: {
    ...noBinding, icon: "music"
  },
});
