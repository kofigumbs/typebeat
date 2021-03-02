/*
 * elements
 */

for (const row of [ 'QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./' ]) {
  const keys = Array.from(row).map(cap => (
    bindingsByModifier.has(cap)
      ? `<div class="key mode" data-cap="${cap}">${Tare.html(bindingsByModifier.get(cap).mode)}</div>`
      : `<div class="key play" data-cap="${cap}"></div>`
  ));
    
  document.body.innerHTML += `<div class="row">${keys.join('')}</div>`;
}

const modes = document.querySelectorAll('.mode');
const plays = document.querySelectorAll('.play');


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
    for (const key of modes) {
      key.classList.toggle('bold', !!mode && key.dataset.cap === mode);
      key.classList.toggle('thin', !!mode && key.dataset.cap !== mode);
    }
  }
  else {
    const handler = bindingsByModifier.get(modifier.current).actions.get(cap);
    if (handler)
      down ? handler.onDown?.() : handler.onUp?.();
  }
};

document.addEventListener('keydown', handleDocumentKey);
document.addEventListener('keyup', handleDocumentKey);
document.addEventListener('keypress', event => event.preventDefault());
