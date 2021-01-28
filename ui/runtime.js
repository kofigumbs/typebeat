/*
 * ffi
 */

const ffiGet = (method) => window[`toUi:${method}`]?.();
const ffiPut = (method, value) => window[`fromUi:${method}`]?.(value|0);


/*
 * elements
 */

const bindingsByCap = {
  Q: { symbol: '#', mode: 'Song' },
  W: { symbol: '[|', mode: 'Sequence' },
  E: { symbol: '~~', mode: 'Sample' },
  R: { symbol: '', mode: 'TBD' },
  T: { symbol: '', mode: 'TBD' },
  A: { symbol: '.-', mode: 'Source' },
  S: { symbol: '>>=', mode: 'Filter' },
  D: { symbol: '/=/', mode: 'Envelope' },
  F: { symbol: '=:=', mode: 'Effects' },
  G: { symbol: '=|', mode: 'Tape' },
  Z: { symbol: '', mode: 'TBD' },
  X: { symbol: '', mode: 'TBD' },
  C: { symbol: '', mode: 'TBD' },
  V: { symbol: '', mode: 'TBD' },
  B: { symbol: '', mode: 'TBD' },
};

for (let row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => `
    <div class='key'>
      <div class='cap'>${cap}</div>
      <div class='symbol'>${bindingsByCap[cap]?.symbol || ''}</div>
      <div class='mode'>${bindingsByCap[cap]?.mode || ''}</div>
    </div>
  `);
  document.body.innerHTML += `<div class='row'>${keys.join('')}</div>`;
}

const elementsByCap = {};
for (let element of document.querySelectorAll('.key'))
  elementsByCap[element.querySelector('.cap').innerText] = element;


/*
 * events
 */

let modifiers = [];

const tempo = {
  init() {
    this.state = [];
  },
  handle(event) {
    if (event.type !== 'keydown')
      return;
    this.state.push(event.timeStamp);
    if (this.state.length === 1)
      return;
    let diffs = 0;
    for (let i = 1; i < this.state.length; i++)
      diffs += this.state[i] - this.state[i - 1];
    ffiPut('tempo', Math.round(60000 / (diffs / (this.state.length - 1)) + 1));
  },
};

const setModifiers = values => {
  modifiers = values;
  tempo.init();
  for (let [cap, binding] of Object.entries(bindingsByCap))
    if ('mode' in binding) {
      const element = elementsByCap[cap];
      element.classList.toggle('down', modifiers[0] === cap);
      element.classList.toggle('hidden', !!modifiers[0] && modifiers[0] !== cap);
    }
};

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  const cap = event.code
    .replace('Key', '')
    .replace('Semicolon', ';')
    .replace('Comma', ',').replace('Period', '.').replace('Slash', '/');
  if (bindingsByCap[cap]?.mode && event.type === 'keydown')
    setModifiers([ ...modifiers, cap ]);
  else if (modifiers.includes(cap) && event.type === 'keyup')
    setModifiers(modifiers.filter(x => x !== cap));
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());


/*
 * draw
 */

(async function loop() {
  // TODO
  requestAnimationFrame(loop);
})();
