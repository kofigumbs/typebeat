const rpc = (context, f) => {
  return (method, data = 0) => f('rpc', { context, method, data });
};
const [state, clearCache] = State({
  defaults: [['modifier', undefined], ['tempoTaps', []]],
  get: rpc('get', window.__TAURI__?.invoke ?? console.log),
});
const bindings = Bindings({
  state,
  set: rpc('set', window.__TAURI__?.invoke ?? console.log),
});


/*
 * movable window
 * https://github.com/tauri-apps/wry/blob/134af020c0058a84c8b6d640613d6d001aa42f82/examples/custom_titlebar.rs#L52-L58
 */
document.addEventListener('mousedown', event => {
  if (event.buttons === 1)
    event.detail === 2 ? window.rpc?.notify('maximize') : window.rpc?.notify('move');
})


/*
 * theme
 */

const theme = new Theme();
theme.install();
theme.start();
theme.onLoad = () => {
  for (const [name, value] of Object.entries(theme.active))
    document.documentElement.style.setProperty(`--${name}`, value);
};


/*
 * elements
 */

const capsOnLeft = 'ZXCVBASDFGQWERT';
const capsOnRight = 'NM,./HJKL;YUIOP';

const mapJoin = (iterable, f) => Array.from(iterable).map(f).join('');
document.body.innerHTML += mapJoin(['QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./'], row => `
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
      else if (bindings.has(cap))
        return `
          <div class="key" data-cap="${cap}">
            ${Tare.html(bindings.get(cap).mode)}
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
 * sync
 */

const forceClass = (el, className) => {
  el.classList.remove(className);
  void el.offsetWidth; // trigger a DOM reflow
  el.classList.add(className);
}

const sync = async () => {
  clearCache();
  const binding = bindings.get(state.modifier);
  for (let i = 0; i < keysOnRight.length; i++) {
    const key = keysOnRight[i];
    const action = binding.actions.get(key.dataset.cap);
    labels[i].ariaLabel = await action?.label() ?? '';
    if (!!state.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
    if (await state[`recent ${i}`])
      forceClass(minipads[i], 'pulse');
    minipads[i].classList.toggle('active', i === await state.activeTrack);
  };
  if (state.playing)
    requestSync();
};

let nextSyncId;
const clearNextSyncIdAndSync = () => {
  nextSyncId = null;
  sync();
};
const requestSync = () => {
  if (!nextSyncId)
    nextSyncId = requestAnimationFrame(clearNextSyncIdAndSync);
};


/*
 * events
 */

const hasModifier = event => (
  event.ctrlKey || event.metaKey || event.shiftKey || event.altKey
);

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const handleDocumentKey = event => {
  if (hasModifier(event))
    return;
  event.preventDefault();
  if (event.repeat)
    return;
  const cap = capsByEventCode.get(event.code);
  if (!cap)
    return;
  const down = event.type === 'keydown';
  const binding = bindings.has(cap);
  if (binding && !down)
    return;
  if (binding) {
    state.modifier = state.modifier === cap ? undefined : cap;
    for (const key of keysOnLeft)
      key.classList.toggle('mode', !!state.modifier && key.dataset.cap === state.modifier);
    state.tempoTaps = [];
  }
  else {
    const handler = bindings.get(state.modifier).actions.get(cap);
    down ? handler?.onDown(event.timeStamp) : handler?.onUp(event.timeStamp);
    if (down)
      forceClass(keysOnRight.find(key => cap === key.dataset.cap), 'pulse');
  }
  requestSync();
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => !hasModifier(event));

sync();
