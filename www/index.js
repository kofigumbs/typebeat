import 'firacode';
import Tare from 'tare';
import './index.css';
import './typed-label.js';
import { modes, bindActions } from './bindings.js';


/*
 * Mount elements (this is eager)
 */

const capsOnLeft = 'ZXCVBASDFGQWERT';
const capsOnRight = 'NM,./HJKL;YUIOP';
const mapJoin = (iterable, f) => Array.from(iterable).map(f).join('');
document.querySelector('.mount').innerHTML += mapJoin(['QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./'], row => `
  <div class="row">
    ${mapJoin(row, cap => {
      if (cap === 'Q')
        return `
          <button class="key" data-cap="${cap}">
            ${mapJoin(capsOnRight.match(/.{1,5}/g).reverse(), minirow => `
              <div class="minirow">
                ${mapJoin(minirow, c => `<div class="minipad" data-cap="${c}"></div>`)}
              </div>`
            )}
          </button>
        `;
      else if (modes.has(cap))
        return `
          <button class="key" data-cap="${cap}">
            ${Tare.html(modes.get(cap))}
          </button>
        `;
      else
        return `
          <button class="key pad" data-cap="${cap}">
            <typed-label class="label" aria-label=""></typed-label>
          </button>
        `;
    })}
  </div>
`);

const findElements = (caps, f) => Array.from(caps).map(cap => document.querySelector(f(cap)));
const keysOnLeft = findElements(capsOnLeft, cap => `.key[data-cap="${cap}"]`);
const keysOnRight = findElements(capsOnRight, cap => `.key[data-cap="${cap}"]`);
const labels = findElements(capsOnRight, cap => `.key[data-cap="${cap}"] .label`);
const minipads = findElements(capsOnRight, cap => `.minipad[data-cap="${cap}"]`);


/*
 * Sync the UI with the proxy state
 */

const unpulse = event => event.target.classList.remove('pulse');
const pulse = el => {
  el.classList.remove('pulse');
  void el.offsetWidth; // trigger a DOM reflow
  el.classList.add('pulse');
}

const sync = async (state) => {
  const { local, proxy, actions } = state;
  proxy.invalidate();
  const mode = modes.get(local.modifier);
  for (let i = 0; i < capsOnRight.length; i++) {
    const action = actions.get(mode).get(capsOnRight[i]);
    labels[i].setAttribute('aria-label', await action?.label() ?? '');
    if (!!local.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
    if (await proxy[`recent ${i}`])
      pulse(minipads[i]);
    minipads[i].classList.toggle('active', i === await proxy.activeTrack);
  };
  if (await proxy.playing)
    requestSync(state);
};

let nextSyncId;
const clearNextSyncIdAndSync = (state) => {
  nextSyncId = null;
  sync(state);
};
const requestSync = (state) => {
  if (!nextSyncId)
    nextSyncId = requestAnimationFrame(() => clearNextSyncIdAndSync(state));
};


/*
 * Event listeners
 */

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
  const { local, proxy, actions } = state;
  if (!modes.has(cap)) {
    const action = actions.get(modes.get(local.modifier)).get(cap);
    down ? action?.onDown(event.timeStamp) : action?.onUp(event.timeStamp);
    if (down)
      pulse(keysOnRight.find(key => cap === key.dataset.cap));
  }
  else if (down) {
    local.modifier = local.modifier === cap ? undefined : cap;
    for (const key of keysOnLeft)
      key.classList.toggle('mode', !!local.modifier && key.dataset.cap === local.modifier);
    local.tempoTaps = [];
  }
  requestSync(state);
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
  event.preventDefault();
  handleCap(event, cap, state);
};

export default (callback) => {
  const local = { tempoTaps: [] };
  const proxyCache = new Map();
  const proxy = new Proxy({}, {
    get: (self, method) => {
      if (method === "invalidate")
        return proxyCache.clear.bind(proxyCache);
      if (!proxyCache.has(method))
        proxyCache.set(method, callback('get', { method }));
      return proxyCache.get(method);
    },
  });
  const set = (method, data = 0) => callback('set', { method, data });
  const state = { local, proxy, actions: bindActions(local, proxy, set) };
  document.addEventListener('keydown', event => handleDocumentKey(event, state));
  document.addEventListener('keyup', event => handleDocumentKey(event, state));
  document.addEventListener('keypress', event => !getCap(event));
  for (let key of document.querySelectorAll('.key')) {
    key.addEventListener('pointerdown', event => handlePointer(event, key.dataset.cap, state));
    key.addEventListener('pointerup', event => handlePointer(event, key.dataset.cap, state));
  }
  minipads.forEach(el => el.addEventListener('animationend', unpulse));
  keysOnRight.forEach(el => el.addEventListener('animationend', unpulse));
  sync(state);
};
