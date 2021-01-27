/*
 * ffi
 */

const ffiGet = (method) => window[`toUi:${method}`]?.();
const ffiPut = (method, value) => window[`fromUi:${method}`]?.(value|0);


/*
 * events
 */

let modifiers = [];

const tapTempo = {
  init() {
    this.state = [];
  },
  handle(event) {
    if (event.type !== "keydown")
      return;
    this.state.push(event.timeStamp);
    if (this.state.length === 1)
      return;
    let diffs = 0;
    for (let i = 1; i < this.state.length; i++)
      diffs += this.state[i] - this.state[i - 1];
    ffiPut("tempo", Math.round(60000 / (diffs / (this.state.length - 1)) + 1));
  },
};

const keys = document.querySelectorAll("[data-cap]");
const keysByEventCode = Object.fromEntries(Array.from(keys).map(key => [
  key.dataset.cap
    .replace(/[a-z]/, match => `Key${match.toUpperCase()}`)
    .replace(";", "Semicolon")
    .replace(",", "Comma").replace(".", "Period").replace("/", "Slash"),
  key,
]));

const setModifiers = values => {
  modifiers = values;
  tapTempo.init();
  for (let key of keys)
    if (key.dataset.role === "modify") {
      key.classList.toggle("down", modifiers[0] === key.dataset.cap);
      key.classList.toggle("gone", !!modifiers[0] && modifiers[0] !== key.dataset.cap);
    }
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  if (key.dataset.role === "modify" && !modifiers.includes(key.dataset.cap) && event.type === "keydown")
    setModifiers([ ...modifiers, key.dataset.cap ]);
  else if (modifiers.includes(key.dataset.cap) && event.type === "keyup")
    setModifiers(modifiers.filter(x => x !== key.dataset.cap));
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
 * draw
 */

(async function loop() {
  const active = {};
  for (let method of getMethods)
    active[method] = await ffiGet(method);

  // TODO

  requestAnimationFrame(loop);
})();
