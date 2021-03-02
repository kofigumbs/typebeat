/*
 * theme
 */

const theme = new Theme();
theme.install();
theme.start();


/*
 * bindings
 */

const bindKeys = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByCap = new Map([
  ['Q', { mode: 'Sample', actions: new Map([
  ])}],
  ['W', { mode: 'Osc.', actions: new Map([
  ])}],
  ['E', { mode: 'LFO', actions: new Map([
  ])}],
  ['R', { mode: 'Select', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('selectVoice', i),
    })),
  ])}],
  ['T', { mode: 'Note', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('noteDown', i),
      onUp: () => $send('noteUp', i),
    })),
  ])}],
  ['A', { mode: 'Seq.', actions: new Map([
  ])}],
  ['S', { mode: 'Filter', actions: new Map([
  ])}],
  ['D', { mode: 'Hold', actions: new Map([
  ])}],
  ['F', { mode: 'FX', actions: new Map([
  ])}],
  ['G', { mode: 'Tape', actions: new Map([
  ])}],
  ['Z', { mode: 'File', actions: new Map([
  ])}],
  ['X', { mode: '_', actions: new Map([
  ])}],
  ['C', { mode: '_', actions: new Map([
  ])}],
  ['V', { mode: '_', actions: new Map([
  ])}],
  ['B', { mode: 'Mute', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      onDown: () => $send('mute', i),
    })),
  ])}],
  [undefined, { actions: new Map([
      ...bindKeys('NM,./HJKL;YUIOP', i => ({
        onDown: () => $send('auditionDown', i),
        onUp: () => $send('auditionUp', i),
      })),
  ])}],
]);


/*
 * elements
 */

for (const row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => (
    bindingsByCap.has(cap)
      ? `<div class="key" data-cap="${cap}">${Tare.html(bindingsByCap.get(cap).mode)}</div>`
      : `<div class="key" data-cap="${cap}"><div class="box"></div></div>`
  ));
  document.body.innerHTML += `<div class="row">${keys.join('')}</div>`;
}

const keysOnLeft = Array.from("ZXCVBASDFGQWERT").map(cap => document.querySelector(`[data-cap="${cap}"]`));
const keysOnRight = Array.from("NM,./HJKL;YUIOP").map(cap => document.querySelector(`[data-cap="${cap}"]`));


/*
 * events
 */

const modifier = {
  down: new Set(),

  toggle(element, keep) {
    keep ? this.down.add(element) : this.down.delete(element);
    return this;
  },

  get current() {
    const [current] = this.down;
    return current;
  },
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
  if (bindingsByCap.has(cap)) {
    const mode = modifier.toggle(cap, down).current;
    for (const key of keysOnLeft) {
      key.classList.toggle('bold', !!mode && key.dataset.cap === mode);
      key.classList.toggle('thin', !!mode && key.dataset.cap !== mode);
    }
  }
  else {
    try {
      const handler = bindingsByCap.get(modifier.current).actions.get(cap);
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

(async function loop() {
  try {
    const selectedVoice = await $receive("selectedVoice");
    keysOnRight.forEach((key, i) => key.classList.toggle('selected', i === selectedVoice));
  } catch {
  }
  requestAnimationFrame(loop)
})();
