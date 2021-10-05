import bind from '../bind';
import grid from './track-grid';

export const cap = 'B';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => await proxy[`muted ${i}`] ? '██' : '--',
    onDown: () => set('muted', i),
  })),
]);

customElements.define('mute-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = grid({ scope: 'mute-mode' });
    this._tracks = Array.from({ length: 15 }).map((_, i) => this.querySelector(`#track-${i}`));
  }

  async sync({ proxy }) {
    this._tracks.forEach(async (track, i) => {
      track.classList.toggle('active', await proxy[`muted ${i}`]);
    });
  }
});
