/*
 * elements
 */

document.body.innerHTML = `
  <div class="screen"></div>
  <div class="keyboard"></div>
`;
const screen = document.querySelector('.screen');
const keyboard = document.querySelector('.keyboard');

const drawWaveform = () => {
  $waveform().then(waveform => {
    screen.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${waveform.length-1} 40" style="width: 100%; height: 100%;">
      <path d="M ${waveform.map((x,i) => `${i} ${(x+1) * 20}`).join(' L ')}" stroke-width=".1"></path>
      </svg>
    `;
  });
};
setTimeout(drawWaveform, 0);

for (let row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => `
    <div class='key'>
      <div class='cap'>${cap}</div>
      <div class='symbol'>${bindingsByModifier.get(cap)?.symbol || ''}</div>
      <div class='mode'>${bindingsByModifier.get(cap)?.mode || ''}</div>
    </div>
  `);
  keyboard.innerHTML += `<div class='row'>${keys.join('')}</div>`;
}

const elementsByCap = new Map();
for (let element of document.querySelectorAll('.key'))
  elementsByCap.set(element.querySelector('.cap').innerText, element);


/*
 * events
 */

const modifier = {
  down: new Set(),

  toggle(element, keep) {
    keep ? this.down.add(element) : this.down.delete(element);
    return this;
  },

  get mode() {
    const [mode] = this.down;
    return mode;
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
  elementsByCap.get(cap).classList.toggle('down', down);
  if (bindingsByModifier.has(cap)) {
    const mode = modifier.toggle(cap, down).mode;
    for (let [cap, binding] of bindingsByModifier)
      elementsByCap.get(cap)?.classList.toggle('hidden', !!mode && mode !== cap);
  }
  else {
    const handler = bindingsByModifier.get(modifier.mode).actions.get(cap);
    if (handler)
      down ? handler.onDown?.() : handler.onUp?.();
    drawWaveform();
  }
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());
