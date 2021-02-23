/*
 * elements
 */

let right = '';
for (let row of [ 'YUIOP', 'HJKL;', 'NM,./' ]) {
  const keys = Array.from(row).map(cap => `
    <div class='key'>
      <div class='box'></div>
      <div class='cap'>${cap}</div>
    </div>
  `);
  right += `<div class='row'>${keys.join('')}</div>`;
}
document.body.innerHTML += `
  <div class='screen'><canvas class='illustrations'></canvas></div>
  <div class='keys'>${right}</div>
`;

const keysByCap = new Map();
for (let element of document.querySelectorAll('.key'))
  keysByCap.set(element.querySelector('.cap').innerText, element);


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
  if (bindingsByModifier.has(cap)) {
    const mode = modifier.toggle(cap, down).current;
    // TODO focus illustration
  }
  else {
    keysByCap.get(cap).classList.toggle('down', down);
    const handler = bindingsByModifier.get(modifier.current).actions.get(cap);
    if (handler)
      down ? handler.onDown?.() : handler.onUp?.();
  }
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());
