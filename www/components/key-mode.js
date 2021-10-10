import bind from '../bind';

export const cap = 'Z';

export const actions = (local, proxy, set) => new Map([
  ['Y', bind.title(() => 'root')],
  ['K', bind.title(async () => bind.note(await proxy.root + 12)) ],
  ...bind.group('HJL;', i => ({
    label: () => ['-5th', '-1/2', '+1/2', '+5th'][i],
    onDown: () => set('root', i),
  })),
  ...bind.group('NM,.', i => ({
    label: () => ['major', 'minor', 'harm.', 'melodic'][i],
    title: async () => i === await proxy.scale,
    onDown: () => set('scale', i),
  })),
]);

customElements.define('key-mode', class extends HTMLElement {
  scales = [
    'm 0 -5.1 l 6 10.2 h -12 l 6 -10.2 Z',
    'm -6 -5.1 h 12 l -6 10.2 Z',
    'm -6 6.9 v -12 v 6 h 12 v -6 v 12',
    'm -6 6.9 v -10 l 6 8.2 l 6 -8.2 v 10',
  ];

  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          key-mode.visual .none {
            fill: none;
          }
          key-mode.visual .background {
            fill: var(--key_background);
          }
        </style>
        <circle r="12" cx="48" cy="23" class="background" stroke-width="2"></circle>
        <path stroke-width="2"></path>
      </svg>
    `;
    const circle = this.querySelector('circle');
    this._r = +circle.getAttribute('r');
    this._cx = +circle.getAttribute('cx');
    this._cy = +circle.getAttribute('cy');
    this._path = this.querySelector('path');
  }

  async sync({ proxy }) {
    const d = this.scales[await proxy.scale];
    const t = await proxy.root / 6 * Math.PI;
    const x = this._cx + Math.sin(t)*this._r;
    const y = this._cy - Math.cos(t)*this._r;
    this._path.setAttribute('d', `M ${x} ${y} ${d}`);
    this._path.setAttribute('class', d.endsWith('Z') ? 'background' : 'none');
  }
});
