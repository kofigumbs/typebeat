import bind from '../bind';

export const cap = 'T';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => bind.note(await proxy[`note ${i}`]),
    title: async () => i == await proxy.activeKey,
    onDown: () => set('noteDown', i),
    onUp: () => set('noteUp', i),
  })),
]);

customElements.define('note-mode', class extends HTMLElement {
  // DOM ids in z order (white keys then black keys)
  ids = [
    "note-0", "note-2", "note-4", "note-5", "note-7", "note-9", "note-11",
    "note-1", "note-3",           "note-6", "note-8", "note-10",
  ];

  connectedCallback() {
    const whiteWidth = 14;
    const whiteHeight = 48;
    const blackWidth = 10;
    const key = (x, w, h) => `
      <rect id="${this.ids.shift()}" x="${x}" height="${h}" width="${w}" fill="none"></rect>
    `;
    const white = i => key(i*whiteWidth, whiteWidth, whiteHeight);
    const black = i => key((i+1)*whiteWidth - blackWidth/2, blackWidth, whiteHeight/2);
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          note-mode rect {
            stroke: var(--dark);
            fill: var(--key_background);
            transform: translate(-1px, -1px);
          }
          note-mode rect.active {
            fill: var(--secondary);
          }
        </style>
        ${Array.from({ length: 7 }).map((_, i) => white(i)).join('')}
        ${Array.from({ length: 2 }).map((_, i) => black(i)).join('')}
        ${Array.from({ length: 3 }).map((_, i) => black(i + 3)).join('')}
      </svg>
    `;
    this._notes = Array.from({ length: 12 }).map((_, i) => this.querySelector(`#note-${i}`));
  }

  async sync({ proxy }) {
    const activeNote = await proxy[`note ${await proxy.activeKey}`] % 12;
    this._notes.forEach((note, i) => note.classList.toggle('active', i === activeNote));
  }
});
