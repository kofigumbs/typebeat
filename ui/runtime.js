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
