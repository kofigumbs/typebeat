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
  const label = path.match(basename)[1];
  modes.set(module.cap, { label, ...module });
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
          ${Tare.html(modes.get(cap).label)}
          <${modes.get(cap).label}-mode class="visual"></${modes.get(cap).label}>
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
 * Sync the UI with the proxy state, with a 24-frame catchup period
 */

const sync = async (state, catchup = 0) => {
  const { local, proxy, actions } = state;
  proxy.invalidate();
  const mode = modes.get(local.modifier);
  for (let visual of visuals)
    visual.sync?.(state);
  for (let i = 0; i < capsOnRight.length; i++) {
    const action = actions.get(mode.label).get(capsOnRight[i]);
    labels[i].setAttribute('aria-label', await action?.label() ?? '');
    if (!!local.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
  };
  if (catchup > 0 || await proxy.playing)
    requestSync(state, catchup - 1);
};

let nextSyncId, nextCatchup;
const clearNextSyncIdAndSync = (state) => {
  nextSyncId = null;
  sync(state, nextCatchup);
};
const requestSync = (state, catchup = 24) => {
  nextCatchup = catchup;
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
    const action = actions.get(modes.get(local.modifier).label).get(cap);
    down ? action?.onDown(event.timeStamp) : action?.onUp(event.timeStamp);
    if (down)
      pulse(keysOnRight.find(key => cap === key.dataset.cap));
  }
  else if (down) {
    local.modifier = local.modifier === cap ? undefined : cap;
    for (const key of keysOnLeft)
      key.classList.toggle('active', !!local.modifier && key.dataset.cap === local.modifier);
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
  if (event.button)
    return; // don't hijack right-click
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
  const state = { local, proxy, actions: new Map() };
  for (let [cap, mode] of modes.entries())
    state.actions.set(mode.label, mode.actions(local, proxy, set));
  document.addEventListener('keydown', event => handleDocumentKey(event, state));
  document.addEventListener('keyup', event => handleDocumentKey(event, state));
  document.addEventListener('keypress', event => !getCap(event));
  for (let key of document.querySelectorAll('.key')) {
    key.addEventListener('pointerdown', event => handlePointer(event, key.dataset.cap, state));
    key.addEventListener('pointerup', event => handlePointer(event, key.dataset.cap, state));
  }
  sync(state);
  return (id, method) => console.log(id, method);
};
