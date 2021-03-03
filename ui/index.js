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

const g = {}; // global mutable state 🙈

const bind = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByModifier = new Map([
  ['Q', { actions: new Map([
    ...bind('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('selectVoice', i),
      label: () => i === g.selectedVoice && 'active',
    })),
  ])}],
  ['W', { mode: 'In-A', actions: new Map([
  ])}],
  ['E', { mode: 'In-B', actions: new Map([
  ])}],
  ['R', { mode: 'LFO', actions: new Map([
  ])}],
  ['T', { mode: 'Note', actions: new Map([
    ...bind('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('noteDown', i),
      onUp: () => $send('noteUp', i),
    })),
  ])}],
  ['A', { mode: 'Seq.', actions: new Map([
  ])}],
  ['S', { mode: 'Filter', actions: new Map([
  ])}],
  ['D', { mode: 'Env.', actions: new Map([
  ])}],
  ['F', { mode: 'FX', actions: new Map([
  ])}],
  ['G', { mode: 'Tape', actions: new Map([
  ])}],
  ['Z', { mode: 'Proj.', actions: new Map([
  ])}],
  ['X', { mode: 'Song', actions: new Map([
  ])}],
  ['V', { mode: 'Mute', actions: new Map([
    ...bind('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('mute', i),
    })),
  ])}],
  ['C', { mode: 'Chain', actions: new Map([
  ])}],
  ['B', { mode: 'Fill', actions: new Map([
  ])}],
  [undefined, { actions: new Map([
    ...bind('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('auditionDown', i),
      onUp: () => $send('auditionUp', i),
    })),
  ])}],
]);


/*
 * elements
 */

for (const row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => {
    if (cap === 'Q')
      return `
        <div class="key" data-cap="${cap}">
          ${`<div class="minirow">${'<div class="minipad"></div>'.repeat(5)}</div>`.repeat(3)}
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
  });
  document.body.innerHTML += `<div class="row">${keys.join('')}</div>`;
}

const keysOnLeft = Array.from("ZXCVBASDFGQWERT").map(cap => document.querySelector(`[data-cap="${cap}"]`));
const keysOnRight = Array.from("NM,./HJKL;YUIOP").map(cap => document.querySelector(`[data-cap="${cap}"]`));


/*
 * events
 */

const modifiersDown = new Set();
const modifierToggle = (element, keep) => {
  keep ? modifiersDown.add(element) : modifiersDown.delete(element);
  [g.modifier] = modifiersDown;
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
      key.classList.toggle('hold', !!g.modifier && key.dataset.cap === g.modifier);
  }
  else {
    try {
      const handler = bindingsByModifier.get(g.modifier).actions.get(cap);
      down ? handler.onDown() : handler.onUp();
    } catch {
    }
    if (down) {
      const key = document.querySelector(`[data-cap="${cap}"]`);
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

const labels = keysOnRight.map(key => key.querySelector('typed-label'));
const minipads = Array.from(document.querySelectorAll('.minipad'))
  .map((_, i, a) => a[[10, 11, 12, 13, 14, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4][i]]); // reorder to start at bottom left

let save;
(async function loop() {
  try {
    const lastSave = save;
    g.selectedVoice = await $receive("selectedVoice");
    if (lastSave !== (save = JSON.stringify(g))) {
      const binding = bindingsByModifier.get(g.modifier);
      keysOnRight.forEach((key, i) => {
        const action = binding.actions.get(key.dataset.cap);
        labels[i].ariaLabel = action?.label?.() || '';
        minipads[i].classList.toggle('selected', i === g.selectedVoice);
      });
    }
  } catch {
  }
  requestAnimationFrame(loop)
})();
