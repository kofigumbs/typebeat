/*
 * native ffi
 */

const nativeGet = (method) => window[`fromNative:${method}`]();
const nativePut = (method, value) => window[`toNative:${method}`](value|0);


/*
 * bindings dsl
 */

const noModifier = "";
const noActiveValue = null;

const toggle = (method, label, value = 1) => ({ type: "toggle", method, label, value });
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
      nativePut(this.method, Math.round(60000 / (diffs / (this.state.length - 1)) + 1));
    },
  },
};


/*
 * events
 */

let modifier = noModifier;
const bindings = window.bindings();

const keys = document.querySelectorAll(".key");
const keysInPage = document.querySelectorAll(".page");
const keysInSequence = document.querySelectorAll(".sequence");

const keysByEventCode = Object.fromEntries(Array.from(keys).map(key => [
  key.dataset.after
    .replace(/[a-z]/, match => `Key${match.toUpperCase()}`)
    .replace(";", "Semicolon")
    .replace(",", "Comma").replace(".", "Period").replace("/", "Slash"),
  key,
]));

const resetModifier = () => {
  modifier = noModifier;
  custom.bpm.state = [];
}

const handleSend = (event, value) => {
  const binding = bindings[modifier][1][value];
  if (!binding)
    return;
  if (binding.type === "toggle")
    return nativePut(binding.method, event.type === "keydown");
  if (binding.type === "set")
    return nativePut(binding.method, (binding.value + 1) * (event.type === "keydown"));
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
  if (keysByEventCode[event.code])
    handleKeyboardKey(event, keysByEventCode[event.code]);
};

document.addEventListener("keydown", handleDocumentKey);
document.addEventListener("keyup", handleDocumentKey);
document.addEventListener("keypress", event => event.preventDefault());


/*
 * draw loop
 */

let bpm;
const getMethods = new Set([ "beat", "page" ]);
for (let controls of Object.values(bindings)) {
  for (let binding of Object.values(controls[1])) {
    if (binding.method === "bpm")
      bpm = binding;
    getMethods.add(binding.method);
  }
}

(async function loop() {
  const active = {};
  for (let method of getMethods)
    active[method] = await nativeGet(method);
  for (let key of keys) {
    const binding = bindings[modifier][1][key.dataset.after];
    key.classList.toggle("active", binding?.value === (active[binding?.method] ?? "inactive"));
  }

  bpm.label = active.bpm;
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
