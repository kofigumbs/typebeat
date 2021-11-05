import Actions from '../actions';

export const cap = 'G';

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'main',   actions: Actions.nudge('activeTrack', 'main'  ) },
  { cap: 'U', label: 'pan',    actions: Actions.nudge('activeTrack', 'pan'   ) },
  { cap: 'I', label: 'echo',   actions: Actions.nudge('activeTrack', 'echo'  ) },
  { cap: 'O', label: 'reverb', actions: Actions.nudge('activeTrack', 'reverb') },
  { cap: 'P', label: 'drive',  actions: Actions.nudge('activeTrack', 'drive' ) }
)

customElements.define('mix-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          mix-mode.visual * {
            fill: var(--key_background);
          }
        </style>
        <rect></rect>
        <rect></rect>
        <rect></rect>
      </svg>
    `;
    this._rects = this.querySelectorAll('rect');
  }

  sync(state) {
    const s = 24 * state.activeTrack.main / 50;
    const x = 48 - s/2 + state.activeTrack.pan;
    const y = 23 - s/2;
    const r = `${state.activeTrack.reverb / 2}%`;
    const spacing = state.activeTrack.echo / 4;
    const strokeWidth = state.activeTrack.drive + 2;
    for (let i = 0; i < this._rects.length; i++) {
      const rect = this._rects[i];
      rect.setAttribute('x', x + (i-1)*spacing);
      rect.setAttribute('y', y + (i-1)*spacing);
      rect.setAttribute('rx', r);
      rect.setAttribute('ry', r);
      rect.setAttribute('width', s);
      rect.setAttribute('height', s);
      rect.setAttribute('stroke-width', strokeWidth);
    }
  }
});
