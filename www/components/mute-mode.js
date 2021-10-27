import bind from '../bind';
import { inert, muted } from './track-grid';

export const cap = 'B';

export const actions = (state) => new Map([
  ...bind.all(i => ({
    label: () => state.tracks[i].muted ? '</>' : '--',
    onDown: () => state.send('muted', i),
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

  sync(state) {
    this._tracks.forEach((track, i) => {
      track.setAttribute('d', state.tracks[i].muted ? muted[i] : inert[i]);
    });
  }
});
