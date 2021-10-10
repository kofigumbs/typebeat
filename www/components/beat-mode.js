import bind from '../bind';

export const cap = 'A';

export const actions = (local, proxy, set) => new Map([
  ['Y', bind.title(() => 'tempo')],
  ...bind.nudge(() => proxy.tempo, i => set('tempo', i)),
  ['N', bind.toggle('play', () => proxy.playing, () => set('playing')) ],
  ['M', bind.toggle('record', () => proxy.recording, () => set('recording')) ],
  [',', bind.one({
    label: () => 'tap',
    title: () => !!local.tempoTaps.length,
    onDown: (time) => {
      local.tempoTaps.push(time);
      if (local.tempoTaps.length === 1)
        return;
      let diffs = 0;
      for (let i = 1; i < local.tempoTaps.length; i++)
        diffs += local.tempoTaps[i] - local.tempoTaps[i - 1];
      set('tempoTaps', Math.round(60000 / (diffs / (local.tempoTaps.length - 1)) + 1));
    },
  })],
]);

customElements.define('beat-mode', class extends HTMLElement {
  connectedCallback() {
    const y = 23;
    const s = 12;
    const st = s*1.46;
    const cx = s+st + 6;
    const offset = (96-cx-s)/2;
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          beat-mode.visual * {
            fill: none;
          }
          beat-mode.visual .active {
            fill: var(--secondary);
          }
        </style>
        <path d="M ${offset} ${y-s} l ${st} ${s} l -${st} ${s} Z" " stroke-width="2"></path>
        <circle r="${s}" cx="${offset+cx}" cy="${y}" stroke-width="2"></circle>
      </svg>
    `;
    this._play = this.querySelector('path');
    this._record = this.querySelector('circle');
  }

  async sync({ proxy }) {
    this._play.classList.toggle('active', await proxy.playing);
    this._record.classList.toggle('active', await proxy.recording);
  }
});
