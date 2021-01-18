/*
 * ffi
 */

const ffiGet = (method) => window[`toUi:${method}`]?.();
const ffiPut = (method, value) => window[`fromUi:${method}`]?.(value|0);


/*
 * bindings dsl
 */

const noModifier = "";
const noActiveValue = null;
const noBinding = { name: "", icon: "", keyMap: {} };

const toggle = (method, label = "", value = 1) => ({ type: "toggle", method, label, value });
const set = (method, label, value) => ({ type: "set", method, label, value });

const range = (low, high) => {
  return Array.from({ length: high - low + 1 }, (_, i) => (low + i).toString());
};

const labeled = (length, keys, method, toLabel) => {
  return Object.fromEntries(Array.from({ length }, (_, i) => [
    keys[i],
    set(method, toLabel(i), i),
  ]));
};

const labelFirst = (keys, method, label) => {
  return labeled(keys.length, keys, method, i => i ? "" : label);
};

const labelAll = (keys, method, labels) => {
  return labeled(Math.min(keys.length, labels.length), keys, method, i => labels[i]);
};

const bottom8 = (method, label) => labelFirst("xcvbnm,.", method, label);
const left12 = (method, labels) => labelAll("xcvbsdfgwert", method, labels);
const left15 = (method, labels) => labelAll("zxcvbasdfgqwert", method, labels);
const right12 = (method, labels) => labelAll("nm,.hjklyuio", method, labels);
const right15 = (method, labels) => labelAll("nm,./hjkl;yuiop", method, labels);

const custom = {
  tempo: {
    type: "custom", method: "tempo", state: [],
    handle(event) {
      if (event.type !== "keydown")
        return;
      this.state.push(event.timeStamp);
      if (this.state.length === 1)
        return;
      let diffs = 0;
      for (let i = 1; i < this.state.length; i++)
        diffs += this.state[i] - this.state[i - 1];
      redraw = true;
      ffiPut(this.method, Math.round(60000 / (diffs / (this.state.length - 1)) + 1));
    },
  },
};


/*
 * events
 */

let modifier = noModifier;
let redraw = true;
const bindings = window.bindings();

const help = document.querySelector(".help");
const keys = document.querySelectorAll(".key");
const keysInPage = document.querySelectorAll(".page");
const keysInSequence = document.querySelectorAll(".sequence");

const keysByEventCode = Object.fromEntries(Array.from(keys).map(key => [
  key.dataset.symbol
    .replace(/[a-z]/, match => `Key${match.toUpperCase()}`)
    .replace(";", "Semicolon")
    .replace(",", "Comma").replace(".", "Period").replace("/", "Slash"),
  key,
]));

const setModifier = value => {
  modifier = value;
  help.innerText = bindings[modifier].name;
  redraw = true;
};

const resetModifier = () => {
  modifier = noModifier;
  custom.tempo.state = [];
  redraw = true;
}

const handleSend = (event, value) => {
  const binding = bindings[modifier].keyMap[value];
  if (!binding)
    return;
  const down = event.type === "keydown";
  if (down)
    help.innerText = bindings[modifier].name;
  if (down && bindings[modifier].name !== binding.method)
    help.innerText += ` › ${binding.method}`;
  if (binding.type === "toggle")
    ffiPut(binding.method, down);
  if (binding.type === "set")
    ffiPut(binding.method, (binding.value + 1) * down);
  if (binding.type === "custom")
    binding.handle(event);
};

const handleModifier = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    setModifier(value);
  else if (modifier === value && event.type === "keyup")
    resetModifier();
  else
    handleSend(event, value);
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "send")
    return handleSend(event, key.dataset.symbol);
  if (key.dataset.control === "modify")
    return handleModifier(event, key.dataset.symbol);
};

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  if (keysByEventCode[event.code])
    handleKeyboardKey(event, keysByEventCode[event.code]);
};

document.addEventListener("keydown", handleDocumentKey);
document.addEventListener("keyup", handleDocumentKey);
document.addEventListener("keypress", event => event.preventDefault());


/*
 * draw loop
 */

let tempo;
const getMethods = new Set([ "beat", "page" ]);
for (let { keyMap } of Object.values(bindings)) {
  for (let binding of Object.values(keyMap)) {
    if (binding.method === "tempo")
      tempo = binding;
    getMethods.add(binding.method);
  }
}

(async function loop() {
  const active = {};
  for (let method of getMethods)
    active[method] = await ffiGet(method);
  for (let key of keys) {
    const binding = bindings[modifier].keyMap[key.dataset.symbol];
    key.classList.toggle("active", binding?.value === (active[binding?.method] ?? "inactive"));
  }

  tempo.label = active.tempo;
  document.body.classList.toggle("recording", active.record);
  keysInPage.forEach((key, i) => key.classList.toggle("available", i <= active.length));
  keysInPage.forEach((key, i) => key.classList.toggle("highlight", i === active.page));
  keysInSequence.forEach((key, i) => key.classList.toggle("highlight", i === active.beat));

  if (redraw) {
    for (const key of keys)
      if (key.dataset.symbol !== modifier) {
        const useIcon = modifier === noModifier && key.dataset.symbol in bindings;
        let html;
        if (!useIcon)
          html = bindings[modifier].keyMap[key.dataset.symbol]?.label;
        else if (bindings[key.dataset.symbol].icon)
          html = `<i data-feather="${bindings[key.dataset.symbol].icon}"></i>`
        key.innerHTML = html || "";
      }
    feather.replace();
    redraw = false;
  }

  requestAnimationFrame(loop);
})();
