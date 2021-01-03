/*
 * native ffi
 */

const nativeGet = (method) => window[`toUi:${method}`]();
const nativePut = (method, value) => window[`fromUi:${method}`](value|0);


/*
 * bindings dsl
 */

const noModifier = "";

const trig = (method, label) => ({ type: "trig", method, label });
const trigValue = (method, value, label) => ({ type: "trigValue", method, value, label });

const range = (low, high) => {
  return Array.from({ length: high - low + 1 }, (_, i) => (low + i).toString());
};

const labeled = (length, keys, method, toLabel) => {
  return Object.fromEntries(Array.from({ length }, (_, i) => [
    keys[i],
    trigValue(method, i, toLabel(i)),
  ]));
};

const labelFirst = (keys, method, label) => {
  return labeled(keys.length, keys, method, i => i ? "" : label);
};

const labelAll = (keys, method, labels) => {
  return labeled(Math.min(keys.length, labels.length), keys, method, i => labels[i]);
};

const top8 = (method, label) => labelFirst("wertyuio", method, label);
const top16 = (method, label) => labelFirst("wertyuiosdfghjkl", method, label);
const middle8 = (method, label) => labelFirst("sdfghjkl", method, label);
const bottom8 = (method, label) => labelFirst("xcvbnm,.", method, label);
const left12 = (method, labels) => labelAll("xcvbsdfgwert", method, labels);
const left15 = (method, labels) => labelAll("zxcvbasdfgqwert", method, labels);
const right12 = (method, labels) => labelAll("nm,.hjklyuio", method, labels);
const right15 = (method, labels) => labelAll("nm,./hjkl;yuiop", method, labels);

const custom = {
  bpm: {
    type: "custom", method: "bpm", state: [],
    handle(event) {
      if (event.type !== "keydown")
        return;
      this.state.push(event.timeStamp);
      if (this.state.length === 1)
        return;
      let diffs = 0;
      for (let i = 1; i < this.state.length; i++)
        diffs += this.state[i] - this.state[i - 1];
      nativePut(this.method, Math.round(60000 / (diffs / (this.state.length - 1))));
    },
  },
};


/*
 * events
 */

let modifier = noModifier;
const bindings = window.bindings();

const keys = document.querySelectorAll(".key");
const keysByCode = Object.fromEntries(Array.from(keys).map(key => [ key.dataset.after, key ]));
const keysInPage = document.querySelectorAll(".page");
const keysInSequence = document.querySelectorAll(".sequence");

const resetModifier = () => {
  custom.bpm.state = [];
  modifier = noModifier;
}

const handleSend = (event, value) => {
  const binding = bindings[modifier][1][value];
  if (!binding)
    return;
  if (binding.type === "trig")
    return nativePut(binding.method, event.type === "keydown");
  if (binding.type === "trigValue")
    return nativePut(`${binding.method}:${binding.value}`, event.type === "keydown");
  if (binding.type === "custom")
    return binding.handle(event, value);
};

const handleModifier = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    resetModifier();
  else
    handleSend(event, value);
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "send")
    return handleSend(event, key.dataset.after);
  if (key.dataset.control === "modify")
    return handleModifier(event, key.dataset.after);
};

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  let code = event.code
    .replace("Semicolon", ";")
    .replace("Comma", ",").replace("Period", ".").replace("Slash", "/")
    .replace("Key", "").toLowerCase();
  if (keysByCode[code])
    handleKeyboardKey(event, keysByCode[code]);
};

document.addEventListener("keydown", handleDocumentKey);
document.addEventListener("keyup", handleDocumentKey);
document.addEventListener("keypress", event => event.preventDefault());


/*
 * draw loop
 */

const getMethods = new Set([ "beat", "page" ]);
for (let controls of Object.values(bindings))
  for (let binding of Object.values(controls[1]))
    getMethods.add(binding.method);

(async function loop() {
  const active = {};
  for (let method of getMethods)
    active[method] = await nativeGet(method);
  for (let key of keys) {
    const binding = bindings[modifier][1][key.dataset.after];
    key.classList.toggle("active", binding?.value === (active[binding?.method] ?? "inactive"));
  }

  bindings["a"][1]["/"]["label"] = active.bpm;
  document.body.classList.toggle("arm", active.arm);
  keysInPage.forEach((key, i) => key.classList.toggle("available", i <= active.length));
  keysInPage.forEach((key, i) => key.classList.toggle("highlight", i === active.page));
  keysInSequence.forEach((key, i) => key.classList.toggle("highlight", i === active.beat));

  for (const key of keys)
    if (key.dataset.after !== modifier)
      key.dataset.before = modifier === noModifier && key.dataset.after in bindings
        ? bindings[key.dataset.after][0]
        : bindings[modifier][1][key.dataset.after]?.label || "";

  requestAnimationFrame(loop);
})();
