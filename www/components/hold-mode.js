import bind from '../bind';

export const cap = 'D';

export const actions = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIOP', 'hold', ['attack', 'decay', 'sustain', 'release', 'cutoff'], local),
  ...bind.nudge(async () => await proxy[local.hold], i => set(local.hold, i)),
  ['N', bind.toggle('sample', async () => await proxy.holdSample, () => set('holdSample')) ],
]);

customElements.define('hold-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          hold-mode path {
            stroke: var(--dark);
          }
        </style>
        <path id="hold" fill="none" stroke-width="2"></path>
      </svg>
    `;
    this._path = this.querySelector('#hold');
  }

  async sync({ proxy }) {
    const a = await proxy.attack;
    const d = await proxy.decay;
    const s = await proxy.sustain;
    const r = await proxy.release;
    this._path.setAttribute('d', `
      M 8 40 l ${a*20/50} -34
      l ${d*20/50} ${34 * (1-s/50)}
      H ${88 - r*20/50}
      L 88 40
    `);
  }
});
