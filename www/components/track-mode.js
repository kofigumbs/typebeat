import bind from '../bind';
import pulse from '../pulse';
import { inert, active } from './track-grid';

export const cap = 'Q';

export const actions = (state) => new Map([
  ...bind.all(i => ({
    label: () => i === state.song.activeTrack ? 'active' : '',
    title: () => !state.song.playing,
    onDown: () => state.send('activeTrack', i),
    onUp: () => state.send('auditionUp', i),
  })),
]);

customElements.define('track-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          track-mode.visual path {
            fill: var(--key_fill);
            --key_stroke: var(--dark);
            --key_fill: var(--secondary);
            --key_pulse: transparent;
          }
        </style>
        ${inert.map(d => `<path d="${d}" stroke-width="2"></path>`).join('')}
      </svg>
    `;
    this._tracks = Array.from(this.querySelectorAll('path'));
  }

  sync(state) {
    if (!this._lastRecent)
      this._lastRecent = state.tracks.map(track => track.recent);
    this._tracks.forEach((track, i) => {
      track.setAttribute('d', i === state.song.activeTrack ? active[i] : inert[i]);
      if (this._lastRecent[i] < state.tracks[i].recent) {
        this._lastRecent[i] = state.tracks[i].recent;
        pulse(track);
      }
    });
  }
});
