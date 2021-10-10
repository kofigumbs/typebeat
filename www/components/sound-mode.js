import bind from '../bind';

export const cap = 'W';

export const actions = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIO', 'sound', ['sample', 'synth 1', 'synth 2', 'synth 3'], local),
  ...bind.oneOf('NM,', 'soundControl', ['type', 'level', 'detune'], local),
  ...bind.group('HJKL;', i => {
    const soundMethod = () => bind.join(local.sound, local.soundControl);
    const soundNudge = bind.nudge(() => proxy[soundMethod()], j => set(soundMethod(), j))[i][1];
    return {
      label: () => {
        if (local.soundControl !== 'type')
          return soundNudge.label();
        else if (local.sound === 'sample')
          return ['file', 'live ->', 'live .=', 'live |>'][i]
        else
          return ['sine', 'tri.', 'saw', 'square', 'noise'][i];
      },
      title: async () => (
        local.soundControl === 'type' ? i === await proxy[soundMethod()] : soundNudge.title()
      ),
      onDown: async () => {
        local.soundControl === 'type' ? set(soundMethod(), i) : soundNudge.onDown();
      },
    };
  }),
]);

const length = 12;
const noise = Array.from({ length }).map(() => Math.random()*2 - 1);

customElements.define('sound-mode', class extends HTMLElement {
  synths = [
    (i, n) => Math.sin(i/n*2*Math.PI),
    (i) => [0, 1, 0, -1][i % 4],
    (i) => [-1, 0, 1][i % 3],
    (i) => [1, 1, -1, -1][i % 4],
    (i) => noise[i],
  ];

  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        ${Array.from({ length }).map((_, i) => `<path stroke-width="2"></path>`).join('')}
      </svg>
    `;
    this._paths = Array.from(this.querySelectorAll('path'));
  }

  async sync({ proxy }) {
    const amplitude = 20 * await proxy.sampleLevel / 50 + 4;
    const synth1 = this.synths[await proxy.synth1Type];
    const synth2 = this.synths[await proxy.synth2Type];
    const synth3 = this.synths[await proxy.synth3Type];
    const offset1 = await proxy.synth1Level/50 * 12
    const offset2 = await proxy.synth2Level/50 * 12
    const offset3 = await proxy.synth3Level/50 * 12
    for (let i = 0; i < length; i++) {
      const x = 47 - 4*(i-length/2);
      const y = 23 - amplitude/2 +
        synth1(i, length) * offset1 +
        synth2(i, length) * offset2 +
        synth3(i, length) * offset3;
      this._paths[i].setAttribute('d', `M ${x} ${y} v ${amplitude}`);
    }
  }
});
