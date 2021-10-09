import bind from '../bind';
import { inert, muted } from './track-grid';

export const cap = 'B';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => await proxy[`muted ${i}`] ? '</>' : '--',
    onDown: () => set('muted', i),
  })),
]);

customElements.define('mute-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        ${inert.map(d => `<path d="${d}" fill="none" stroke-width="2"></path>`).join('')}
      </svg>
    `;
    this._tracks = Array.from(this.querySelectorAll('path'));
  }

  async sync({ proxy }) {
    this._tracks.forEach(async (track, i) => {
      track.setAttribute('d', await proxy[`muted ${i}`] ? muted[i] : inert[i]);
    });
  }
});
