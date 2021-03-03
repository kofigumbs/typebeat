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

const bind = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByModifier = new Map([
  ['Q', { actions: new Map([
    ...bind(capsOnRight, i => ({
      onDown: () => window.$send?.('selectVoice', i),
      label: () => i === state.selectedVoice && 'active',
    })),
  ])}],
  ['W', { mode: 'In-A', actions: new Map([
  ])}],
  ['E', { mode: 'In-B', actions: new Map([
  ])}],
  ['R', { mode: 'LFO', actions: new Map([
  ])}],
  ['T', { mode: 'Note', actions: new Map([
    ...bind(capsOnRight, i => ({
      onDown: () => window.$send?.('noteDown', i),
      onUp: () => window.$send?.('noteUp', i),
    })),
  ])}],
  ['A', { mode: 'Seq.', actions: new Map([
  ])}],
  ['S', { mode: 'Filter', actions: new Map([
  ])}],
  ['D', { mode: 'Env.', actions: new Map([
  ])}],
  ['F', { mode: 'FX', actions: new Map([
    ...bind('YUIOPN', i => ({
      onDown: () => state.submenuFx = i,
      title: () => (state.submenuFx ?? 0) === i,
      label: () => ['pan', 'delay', 'reverb', 'chorus', 'distort', 'volume'][i],
    })),
  ])}],
  ['G', { mode: 'Tape', actions: new Map([
  ])}],
  ['Z', { mode: 'Proj.', actions: new Map([
  ])}],
  ['X', { mode: 'Song', actions: new Map([
  ])}],
  ['V', { mode: 'Mute', actions: new Map([
    ...bind(capsOnRight, i => ({
      onDown: () => window.$send?.('mute', i),
    })),
  ])}],
  ['C', { mode: 'Chain', actions: new Map([
  ])}],
  ['B', { mode: 'Fill', actions: new Map([
  ])}],
  [undefined, { actions: new Map([
    ...bind(capsOnRight, i => ({
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
    down ? handler?.onDown?.() : handler?.onUp?.();
    if (down) {
      const key = keysOnRight.find(key => cap === key.dataset.cap);
      key.classList.remove('pulse');
      void key.offsetWidth; // trigger a DOM reflow
      key.classList.add('pulse');
    }
  }
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());


/*
 * draw loop
 */

let savedState;
(async function loop() {
  state.selectedVoice = await window.$receive?.("selectedVoice");
  if (savedState !== (savedState = JSON.stringify(state))) {
    const binding = bindingsByModifier.get(state.modifier);
    keysOnRight.forEach((key, i) => {
      const action = binding.actions.get(key.dataset.cap);
      labels[i].ariaLabel = action?.label?.() ?? '';
      labels[i].classList.toggle('title', !!(action?.title?.()));
      minipads[i].classList.toggle('active', i === state.selectedVoice);
    });
  }
  requestAnimationFrame(loop)
})();
