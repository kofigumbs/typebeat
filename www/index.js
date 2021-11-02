import { createEffect, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';
import { render } from 'solid-js/web';

import 'firacode';
import Tare from 'tare';
import pulse from './pulse';
import './components/typed-label';
import './index.css';

/*
 * Map caps mode modules
 */

const modes = new Map();
const basename = /\/(\w+)-mode\./;
for (let [path, module] of Object.entries(import.meta.globEager('./components/*-mode.js'))) {
  const name = path.match(basename)[1];
  modes.set(module.cap, { name, ...module });
}


/*
 * Mount elements (this is eager)
 */

const capsOnLeft = 'ZXCVBASDFGQWERT';
const capsOnRight = 'NM,./HJKL;YUIOP';
const mapJoin = (iterable, f) => Array.from(iterable).map(f).join('');
document.querySelector('.mount').innerHTML += mapJoin(['QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./'], row => `
  <div class="row">
    ${mapJoin(row, cap => modes.has(cap)
      ? `
        <button class="key mode" data-cap="${cap}">
          ${Tare.html(modes.get(cap).name)}
          <${modes.get(cap).name}-mode class="visual"></${modes.get(cap).name}>
        </button>
      `
      : `
        <button class="key action" data-cap="${cap}">
          <typed-label class="label" aria-label=""></typed-label>
        </button>
      `
    )}
  </div>
`);

const findElements = (caps, f) => Array.from(caps).map(cap => document.querySelector(f(cap)));
const keysOnLeft = findElements(capsOnLeft, cap => `[data-cap="${cap}"]`);
const keysOnRight = findElements(capsOnRight, cap => `[data-cap="${cap}"]`);
const visuals = findElements(capsOnLeft, cap => `[data-cap="${cap}"] .visual`);
const labels = findElements(capsOnRight, cap => `[data-cap="${cap}"] .label`);

/*
 * Event listeners
 */

const [modifier, setModifier] = createSignal();

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const getCap = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey)
    return;
  return capsByEventCode.get(event.code);
};

const handleCap = (event, cap, state) => {
  const down = event.type.endsWith('down');
  if (!modes.has(cap)) {
    const action = modes.get(modifier()).actions.get(cap);
    down ? action?.onDown(state, event) : action?.onUp(state, event);
    if (down)
      pulse(keysOnRight.find(key => cap === key.dataset.cap));
  }
  else if (down) {
    setModifier(modifier => modifier === cap ? undefined : cap);
    for (const key of keysOnLeft)
      key.classList.toggle('active', !!modifier() && key.dataset.cap === modifier());
    state.tempoTaps = [];
  }
}

const handleDocumentKey = (event, state) => {
  const cap = getCap(event);
  if (cap) {
    event.preventDefault();
    if (!event.repeat)
      handleCap(event, cap, state);
  }
};

const handlePointer = (event, cap, state) => {
  if (event.button)
    return; // don't hijack right-click
  event.preventDefault();
  handleCap(event, cap, state);
};

export default (dump, send) => {
  const [state, setState] = createStore({
    ...dump,
    send,
    actions: new Map(),
    get activeTrack() {
      return this.tracks[this.song.activeTrack];
    }
  });
  document.addEventListener('keydown', event => handleDocumentKey(event, state));
  document.addEventListener('keyup', event => handleDocumentKey(event, state));
  document.addEventListener('keypress', event => !getCap(event));
  for (let key of document.querySelectorAll('.key')) {
    key.addEventListener('pointerdown', event => handlePointer(event, key.dataset.cap, state));
    key.addEventListener('pointerup', event => handlePointer(event, key.dataset.cap, state));
  }
  createEffect(() => {
    const mode = modes.get(modifier());
    for (let visual of visuals)
      visual.sync?.(state);
    for (let i = 0; i < capsOnRight.length; i++) {
      const action = mode.actions.get(capsOnRight[i]);
      labels[i].setAttribute('aria-label', action?.label(state) ?? '');
      if (!!modifier())
        labels[i].classList.toggle('title', !!action?.title(state));
    }
  });
  return ([id, method, value]) => {
    const args = id === 0 ? ['song', method, value] : ['tracks', id-1, method, value];
    setState(...args);
  };
};
