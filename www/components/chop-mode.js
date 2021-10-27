import Actions from '../actions';

export const cap = 'E';

export const actions = Actions.comingSoon;

customElements.define('chop-mode', class extends HTMLElement {
  waveform = f => Array.from({ length: 24 }).map((_, i) => f(i));

  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        ${this.waveform(i => `<path stroke-width="2"></path>`).join('')}
      </svg>
    `;
    this._paths = Array.from(this.querySelectorAll('path'));
  }

  sync(state) {
    this.waveform(i => {
      const path = this._paths[i];
      const amplitude = state.activeTrack[`waveform${i}`]/5 + 1;
      path.setAttribute('d', `M ${i*4 + 3} ${23 - amplitude} v ${amplitude*2}`);
    });
  }
});
