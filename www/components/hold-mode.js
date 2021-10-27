import bind from '../bind';

export const cap = 'D';

export const actions = (state) => new Map([
  ...bind.oneOf('YUIOP', 'hold', ['attack', 'decay', 'sustain', 'release', 'cutoff'], state),
  ...bind.nudge(() => state.activeTrack()[state.hold], i => state.send(state.hold, i)),
  ['N', bind.toggle('sample', () => state.activeTrack().holdSample, () => state.send('holdSample', 0)) ],
]);

customElements.define('hold-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <path fill="none" stroke-width="2"></path>
      </svg>
    `;
    this._path = this.querySelector('path');
  }

  sync(state) {
    const a = state.activeTrack().attack;
    const d = state.activeTrack().decay;
    const s = state.activeTrack().sustain;
    const r = state.activeTrack().release;
    this._path.setAttribute('d', `
      M 3 43 l ${a*22/50} -40
      l ${d*22/50} ${40 * (1-s/50)}
      H ${93 - r*22/50}
      L 93 43
    `);
  }
});
