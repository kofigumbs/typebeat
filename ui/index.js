const foreign = (context, f = console.log) => {
  return (method, data) => f(`${context}:${method}`, data);
};
const [state, clearCache] = State({
  defaults: [['modifier', undefined], ['tempoTaps', []]],
  receive: foreign('receive', window.rpc?.call),
});
const bindings = Bindings({
  state,
  send: foreign('send', window.rpc?.notify),
});


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

const sync = async () => {
  clearCache();
  const binding = bindings.get(state.modifier);
  for (let i = 0; i < keysOnRight.length; i++) {
    const key = keysOnRight[i];
    const action = binding.actions.get(key.dataset.cap);
    labels[i].ariaLabel = await action?.label() ?? '';
    if (!!state.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
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

const modifiersDown = new Set();
const modifierToggle = (element, keep) => {
  keep ? modifiersDown.add(element) : modifiersDown.delete(element);
  [state.modifier] = modifiersDown;
};

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  const cap = capsByEventCode.get(event.code);
  if (!cap)
    return;
  const down = event.type === 'keydown';
  if (bindings.has(cap)) {
    modifierToggle(cap, down);
    for (const key of keysOnLeft)
      key.classList.toggle('hold', !!state.modifier && key.dataset.cap === state.modifier);
    state.tempoTaps = [];
  }
  else {
    const handler = bindings.get(state.modifier).actions.get(cap);
    down ? handler?.onDown(event.timeStamp) : handler?.onUp(event.timeStamp);
    if (down) {
      const key = keysOnRight.find(key => cap === key.dataset.cap);
      key.classList.remove('pulse');
      void key.offsetWidth; // trigger a DOM reflow
      key.classList.add('pulse');
    }
  }
  requestSync();
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());

sync();
