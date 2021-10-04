import bind from '../bind';
import pulse from '../pulse';

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
    const row = ({ x, y, id }) => `
      <rect id="track-${id + 0}" x="${x + 0.00}" y="${y}" width="1.5" height=".25"></rect>
      <rect id="track-${id + 1}" x="${x + 1.75}" y="${y}" width="1.5" height=".25"></rect>
      <rect id="track-${id + 2}" x="${x + 3.50}" y="${y}" width="1.5" height=".25"></rect>
      <rect id="track-${id + 3}" x="${x + 5.25}" y="${y}" width="1.5" height=".25"></rect>
      <rect id="track-${id + 4}" x="${x + 7.00}" y="${y}" width="1.5" height=".25"></rect>
    `;
    const grid = ({ x, y }) =>
      row({ x, y, id: 10 }) +
        row({ x: x + 0.5, y: y + 1.5, id: 5 }) +
        row({ x: x + 1.0, y: y + 3.0, id: 0 });
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 6">
        <style>
          rect {
            fill: var(--key_background);
            --key_background: var(--dark);
            --key_pulse: transparent;
          }
        </style>
        ${grid({ x: 1.125, y: 1.375 })}
        <rect id="active" width="1.5" height="1.5"></rect>
      </svg>
    `;
    this._active = this.querySelector('#active');
    this._tracks = Array.from({ length: 15 }).map((_, i) => this.querySelector(`#track-${i}`));
  }

  async sync({ proxy }) {
    const activeTrack = await proxy.activeTrack;
    const row = parseInt(activeTrack / 5);
    this._active.setAttribute('y', 0.75 + 1.5 * (2 - row));
    this._active.setAttribute('x', 2.125 + 1.75 * (activeTrack % 5) - row * .5);
    for (let i = 0; i < this._tracks.length; i++)
      if (await proxy[`recent ${i}`]) {
        pulse(this._tracks[i]);
        if (activeTrack === i)
          pulse(this._active);
      }
  }
});
