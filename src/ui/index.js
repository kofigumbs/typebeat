import './FiraCode/distr/fira_code.css';
import './index.css';
import './Themes/scripts/lib/theme.js';
import './tare/dist/tare.min.js';
import './typed-label.js';
import { modes, bindActions } from './bindings.js';


/*
 * Setup theme
 */

const theme = new window.Theme();
theme.install();
theme.start();
theme.onLoad = () => {
  for (const [name, value] of Object.entries(theme.active))
    document.documentElement.style.setProperty(`--${name}`, value);
};


/*
 * Map modifier caps to modes
 */


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
          <div class="key" data-cap="${cap}">
            ${mapJoin(capsOnRight.match(/.{1,5}/g).reverse(), minirow => `
              <div class="minirow">
                ${mapJoin(minirow, c => `<div class="minipad" data-cap="${c}"></div>`)}
              </div>`
            )}
          </div>
        `;
      else if (modes.has(cap))
        return `
          <div class="key" data-cap="${cap}">
            ${Tare.html(modes.get(cap))}
          </div>
        `;
      else
        return `
          <div class="key pad" data-cap="${cap}">
            <typed-label class="label" aria-label=""></typed-label>
          </div>
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

const forceClass = (el, className) => {
  el.classList.remove(className);
  void el.offsetWidth; // trigger a DOM reflow
  el.classList.add(className);
}

const sync = async (local, proxy, actions) => {
  proxy.invalidate();
  const mode = modes.get(local.modifier);
  for (let i = 0; i < capsOnRight.length; i++) {
    const action = actions.get(mode).get(capsOnRight[i]);
    labels[i].setAttribute('aria-label', await action?.label() ?? '');
    if (!!local.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
    if (await proxy[`recent ${i}`])
      forceClass(minipads[i], 'pulse');
    minipads[i].classList.toggle('active', i === await proxy.activeTrack);
  };
  if (await proxy.playing)
    requestSync(local, proxy, actions);
};

let nextSyncId;
const clearNextSyncIdAndSync = (local, proxy, actions) => {
  nextSyncId = null;
  sync(local, proxy, actions);
};
const requestSync = (local, proxy, actions) => {
  if (!nextSyncId)
    nextSyncId = requestAnimationFrame(() => clearNextSyncIdAndSync(local, proxy, actions));
};


/*
 * Event listeners
 */

const hasModifier = event => (
  event.ctrlKey || event.metaKey || event.shiftKey || event.altKey
);

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const handleCap = (cap, down, local, proxy, actions) => {
  if (!modes.has(cap)) {
    const action = actions.get(modes.get(local.modifier)).get(cap);
    down ? action?.onDown(event.timeStamp) : action?.onUp(event.timeStamp);
    if (down)
      forceClass(keysOnRight.find(key => cap === key.dataset.cap), 'pulse');
  }
  else if (down) {
    local.modifier = local.modifier === cap ? undefined : cap;
    for (const key of keysOnLeft)
      key.classList.toggle('mode', !!local.modifier && key.dataset.cap === local.modifier);
    local.tempoTaps = [];
  }
  requestSync(local, proxy, actions);
}

const handleDocumentKey = (event, local, proxy, actions) => {
  if (hasModifier(event))
    return;
  event.preventDefault();
  if (event.repeat)
    return;
  const cap = capsByEventCode.get(event.code);
  if (!cap)
    return;
  handleCap(cap, event.type === 'keydown', local, proxy, actions);
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
  const actions = bindActions(local, proxy, set);
  const handler = event => handleDocumentKey(event, local, proxy, actions);
  document.addEventListener('keydown', handler);
  document.addEventListener('keyup', handler);
  document.addEventListener('keypress', event => !hasModifier(event));
  sync(local, proxy, actions);
};
