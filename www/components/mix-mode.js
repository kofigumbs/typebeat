import bind from '../bind';

export const cap = 'G';

export const actions = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIOP', 'mix', ['main', 'pan', 'reverb', 'echo', 'drive'], local),
  ...bind.nudge(async () => await proxy[local.mix], i => set(local.mix, i)),
]);

customElements.define('mix-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          mix-mode.visual * {
            fill: var(--key_background);
          }
        </style>
        <rect></rect>
        <rect></rect>
        <rect></rect>
      </svg>
    `;
    this._rects = this.querySelectorAll('rect');
  }

  async sync({ proxy }) {
    const s = 24 * await proxy.main / 50;
    const x = 48 - s/2 + await proxy.pan;
    const y = 23 - s/2;
    const r = `${await proxy.reverb / 2}%`;
    const spacing = await proxy.echo / 4;
    const strokeWidth = await proxy.drive + 2;
    for (let i = 0; i < this._rects.length; i++) {
      const rect = this._rects[i];
      rect.setAttribute('x', x + (i-1)*spacing);
      rect.setAttribute('y', y + (i-1)*spacing);
      rect.setAttribute('rx', r);
      rect.setAttribute('ry', r);
      rect.setAttribute('width', s);
      rect.setAttribute('height', s);
      rect.setAttribute('stroke-width', strokeWidth);
    }
  }
});
