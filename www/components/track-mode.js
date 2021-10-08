import bind from '../bind';
import pulse from '../pulse';
import { inert, active } from './track-grid';

export const cap = 'Q';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => i === await proxy.activeTrack ? 'active' : '',
    title: async () => !await proxy.playing,
    onDown: () => set('activeTrack', i),
  })),
]);

customElements.define('track-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          track-mode path {
            fill: var(--key_fill);
            --key_stroke: var(--dark);
            --key_fill: var(--secondary);
            --key_pulse: transparent;
          }
        </style>
        ${inert.map(d => `<path d="${d}"></path>`).join('')}
      </svg>
    `;
    this._tracks = Array.from(this.querySelectorAll('path'));
  }

  async sync({ proxy }) {
    const activeTrack = await proxy.activeTrack;
    this._tracks.forEach(async (track, i) => {
      track.setAttribute('d', i === activeTrack ? active[i] : inert[i]);
      if (await proxy[`recent ${i}`])
        pulse(track);
    });
  }
});
