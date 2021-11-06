import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'D';

export const actions = Actions.combine(
  Actions.tabbed(
    { cap: 'Y', label: 'attack',  actions: Actions.nudge('activeTrack', 'attack' ) },
    { cap: 'U', label: 'decay',   actions: Actions.nudge('activeTrack', 'decay'  ) },
    { cap: 'I', label: 'sustain', actions: Actions.nudge('activeTrack', 'sustain') },
    { cap: 'O', label: 'release', actions: Actions.nudge('activeTrack', 'release') },
    { cap: 'P', label: 'cutoff',  actions: Actions.nudge('activeTrack', 'cutoff' ) }
  ),
  Actions.cap('N', {
    label: () => 'sample',
    title: state => state.activeTrack.holdSample,
    onDown: state => state.send('holdSample', 0),
  })
);

export const Visual = props => {
  const commands = createMemo(() => {
    const a = props.state.activeTrack.attack;
    const d = props.state.activeTrack.decay;
    const s = props.state.activeTrack.sustain;
    const r = props.state.activeTrack.release;
    return `
      M 3 43 l ${a*22/50} -40
      l ${d*22/50} ${40 * (1-s/50)}
      H ${93 - r*22/50}
      L 93 43
    `;
  });
  return (
    <svg xmlns="http://www.w3.org/2000/svg">
      <path d={commands()} stroke-width="2"></path>
    </svg>
  );
};
