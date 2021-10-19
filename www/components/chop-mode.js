import bind from '../bind';

export const cap = 'E';

export const actions = (local, proxy, set) => bind.comingSoon;

customElements.define('chop-mode', class extends HTMLElement {
  eachWaveform = f => Array.from({ length: 24 }).map((_, i) => f(i));

  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        ${this.eachWaveform(i => `<path stroke-width="2"></path>`).join('')}
      </svg>
    `;
    this._paths = Array.from(this.querySelectorAll('path'));
  }

  async sync({ proxy }) {
    const waveforms = await Promise.all(this.eachWaveform(i => proxy[`waveform${i}`]));
    this._paths.map((path, i) => {
      const amplitude = waveforms[i]/5 + 1;
      path.setAttribute('d', `M ${i*4 + 3} ${23 - amplitude} v ${amplitude*2}`);
    });
  }
});
