const state = {}; // globally mutable 🙈
const capsOnLeft = 'ZXCVBASDFGQWERT';
const capsOnRight = 'NM,./HJKL;YUIOP';


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
 * bindings
 */

const bindingsByModifier = new Map([
  ['Q', { actions: new Map([
    ...Binding.group(capsOnRight, i => ({
      label: () => i === state.activeVoice ? 'active' : '',
      title: () => i === state.activeVoice,
      onDown: () => window.$send?.('selectVoice', i),
    })),
  ])}],
  ['W', { mode: 'Src-A', actions: new Map([
  ])}],
  ['E', { mode: 'Src-B', actions: new Map([
  ])}],
  ['R', { mode: 'Poly', actions: new Map([
  ])}],
  ['T', { mode: 'Note', actions: new Map([
    ...Binding.group(capsOnRight, i => ({
      label: () => `${["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][state.note[i] % 12]}${(state.note[i] / 12 - 1)|0}`,
      title: () => state.note[i] === state.naturalNote,
      onDown: () => window.$send?.('noteDown', i),
      onUp: () => window.$send?.('noteUp', i),
    })),
  ])}],
  ['A', { mode: 'Loop', actions: new Map([
    ...Binding.tabs('YUI', state, 'loop', ['page', 'zoom', 'length']),
    ...Binding.buttons('HJL;', () => ['-4', '-1', '+1', '+4'], i => {}),
  ])}],
  ['S', { mode: 'EQ', actions: new Map([
    ...Binding.tabs('YUIOP', state, 'eqBand', ['hi pass', 'mid 1', 'mid 2', 'mid 3', 'lo pass']),
    ...Binding.tabs('NM', state, 'eqFilter', ['freq.', 'res.']),
    ...Binding.buttons('HJL;', () => ['-10', '-1', '+1', '+10'], i => {}),
    ['/', Binding.toggle('FILL', () => state.fill, () => {}) ],
  ])}],
  ['D', { mode: 'ADSR', actions: new Map([
    ...Binding.tabs('YUIO', state, 'adsr', ['attack', 'decay', 'sustain', 'release']),
    ...Binding.buttons('HJL;', () => ['-10', '-1', '+1', '+10'], i => {}),
    ['/', Binding.toggle('FILL', () => state.fill, () => {}) ],
  ])}],
  ['F', { mode: 'FX', actions: new Map([
    ...Binding.tabs('YUIOP', state, 'fx', ['comp.', 'distort', 'vocoder', 'chorus', 'duck']),
    ...Binding.buttons('HJL;', () => ['-10', '-1', '+1', '+10'], i => {}),
    ['K', Binding.title(() => state.fx[state.tab.fx]) ],
    ['/', Binding.toggle('FILL', () => state.fill, () => {}) ],
  ])}],
  ['G', { mode: 'Mix', actions: new Map([
    ...Binding.tabs('YUIOPNM', state, 'mix', ['volume', 'send 1', 'send 2', 'send 3', 'send 4', 'pan', 'to duck', 'to tape']),
    ...Binding.buttons('HJL;', () => ['-10', '-1', '+1', '+10'], i => window.$send?.('nudge:mix', state.tab.mix << 4 | i)),
    ['K', Binding.title(() => state.mix[state.tab.mix]) ],
    ['/', Binding.toggle('FILL', () => state.fill, () => {}) ],
  ])}],
  ['Z', { mode: 'Song', actions: new Map([
    ...Binding.tabs('Y', state, 'song', ['tempo']),
    ...Binding.buttons('HJL;', () => ['-10', '-1', '+1', '+10'], i => window.$send?.('nudge:tempo', i)),
    ['K', Binding.title(() => state.tempo) ],
    ['N', Binding.toggle('play', () => state.playing, () => window.$send?.('play')) ],
    ['M', Binding.toggle('arm', () => state.armed, () => window.$send?.('arm')) ],
  ])}],
  ['X', { mode: 'Auto', actions: new Map([
  ])}],
  ['C', { mode: 'Send', actions: new Map([
  ])}],
  ['V', { mode: 'Mute', actions: new Map([
  ])}],
  ['B', { mode: 'Live', actions: new Map([
  ])}],
  [undefined, { actions: new Map([
    ...Binding.group(capsOnRight, i => ({
      onDown: () => window.$send?.('auditionDown', i),
      onUp: () => window.$send?.('auditionUp', i),
    })),
  ])}],
]);


/*
 * elements
 */

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
      else if (bindingsByModifier.has(cap))
        return `
          <div class="key" data-cap="${cap}">
            ${Tare.html(bindingsByModifier.get(cap).mode)}
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

const all = (length, f) => Promise.all(Array.from({ length }, (_, i) => f(i)));
const sync = async () => {
  state.tempo       = await window.$receive?.('tempo');
  state.playing     = await window.$receive?.('playing');
  state.armed       = await window.$receive?.('armed');
  state.activeVoice = await window.$receive?.('activeVoice');
  state.scale       = await window.$receive?.('scale');
  state.naturalNote = await window.$receive?.('naturalNote');
  state.note = await all(15, i => window.$receive?.(`note:${i}`));
  state.eq   = await all(10, i => window.$receive?.(`eq:${i}`));
  state.adsr = await all(4,  i => window.$receive?.(`adsr:${i}`));
  state.mix  = await all(7,  i => window.$receive?.(`mix:${i}`));
  state.fx   = await all(5,  i => window.$receive?.(`fx:${i}`));
  const binding = bindingsByModifier.get(state.modifier);
  for (let i = 0; i < keysOnRight.length; i++) {
    const key = keysOnRight[i];
    const action = binding.actions.get(key.dataset.cap);
    labels[i].ariaLabel = action?.label() ?? '';
    if (!!state.modifier)
      labels[i].classList.toggle('title', !!(action?.title()));
    minipads[i].classList.toggle('active', i === state.activeVoice);
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
  if (bindingsByModifier.has(cap)) {
    modifierToggle(cap, down);
    for (const key of keysOnLeft)
      key.classList.toggle('hold', !!state.modifier && key.dataset.cap === state.modifier);
  }
  else {
    const handler = bindingsByModifier.get(state.modifier).actions.get(cap);
    down ? handler?.onDown() : handler?.onUp();
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
document.addEventListener('keypress', event => {
  event.preventDefault();
  if (event.key === 'r' && (event.ctrlKey || event.metaKey))
    window.location.reload();
});

sync();
