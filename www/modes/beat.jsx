import Actions from '../actions';

export const cap = 'A';

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'tempo', actions: Actions.combine(
    Actions.cap('N', {
      label: () => 'play',
      title: (state) => state.song.playing,
      onDown: (state) => state.send('playing', 0),
    }),
    Actions.cap('M', {
      label: () => 'record',
      title: (state) => state.song.recording,
      onDown: (state) => state.send('recording', 0),
    }),
    // [',', bind.one({
    //   label: () => 'tap',
    //   title: () => !!state.tempoTaps.length,
    //   onDown: (time) => {
    //     state.tempoTaps.push(time);
    //     if (state.tempoTaps.length === 1)
    //       return;
    //     let diffs = 0;
    //     for (let i = 1; i < state.tempoTaps.length; i++)
    //       diffs += state.tempoTaps[i] - state.tempoTaps[i - 1];
    //     state.send('tempoTaps', Math.round(60000 / (diffs / (state.tempoTaps.length - 1)) + 1));
    //   },
    // })],
    Actions.nudge('song', 'tempo'),
  )}
);

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

  sync(state) {
    this._play.classList.toggle('active', state.song.playing);
    this._record.classList.toggle('active', state.song.recording);
  }
});
