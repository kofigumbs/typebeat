/*
 * bindings
 */

const ffiGet = (method) => window[`toUi`]?.(method);
const ffiPut = (method, value) => window['fromUi']?.(method, value|0);

const bindKeys = (caps, f) => new Map(Array.from(caps, (cap, i) => [cap, f(i)]));
const bindingsByModifier = new Map([
  ['Q', { symbol: '#', mode: 'Song', actions: new Map([
  ])}],
  ['W', { symbol: '[|', mode: 'Sequence', actions: new Map([
  ])}],
  ['E', { symbol: '~~', mode: 'Sample', actions: new Map([
  ])}],
  ['R', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['T', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['A', { symbol: '.-', mode: 'Source', actions: new Map([
  ])}],
  ['S', { symbol: '>>=', mode: 'Filter', actions: new Map([
  ])}],
  ['D', { symbol: '/=/', mode: 'Envelope', actions: new Map([
  ])}],
  ['F', { symbol: '=:=', mode: 'Effects', actions: new Map([
  ])}],
  ['G', { symbol: '=|', mode: 'Tape', actions: new Map([
  ])}],
  ['Z', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['X', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['C', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['V', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['B', { symbol: '', mode: '', actions: new Map([
  ])}],
  [undefined, { actions: new Map([
    ...bindKeys("NM,./HJKL;YUIOP", i => down => ffiPut(`key${down ? 'down' : 'up'}`, i))
  ])}],
]);


/*
 * elements
 */

for (let row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => `
    <div class='key'>
      <div class='cap'>${cap}</div>
      <div class='symbol'>${bindingsByModifier.get(cap)?.symbol || ''}</div>
      <div class='mode'>${bindingsByModifier.get(cap)?.mode || ''}</div>
    </div>
  `);
  document.body.innerHTML += `<div class='row'>${keys.join('')}</div>`;
}

const elementsByCap = new Map();
for (let element of document.querySelectorAll('.key'))
  elementsByCap.set(element.querySelector('.cap').innerText, element);


/*
 * events
 */

let modifiers = [];

const modify = values => {
  modifiers = values;
  for (let [cap, binding] of bindingsByModifier)
    elementsByCap.get(cap)?.classList.toggle('hidden', !!modifiers[0] && modifiers[0] !== cap);
};

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  const cap = event.code
    .replace('Key', '')
    .replace('Semicolon', ';')
    .replace('Comma', ',').replace('Period', '.').replace('Slash', '/');
  if (elementsByCap.has(cap)) {
    const down = event.type === 'keydown';
    elementsByCap.get(cap).classList.toggle('down', down);
    if (bindingsByModifier.has(cap))
      modify(down ? [ ...modifiers, cap ] : modifiers.filter(x => x !== cap));
    else if (bindingsByModifier.get(modifiers[0]).actions.has(cap))
      bindingsByModifier.get(modifiers[0]).actions.get(cap)(down);
  }
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());
