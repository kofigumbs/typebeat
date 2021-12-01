import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'S';

export const commands = Commands.combine(
  Commands.tabbed(
    { cap: 'Y', label: 'attack',  commands: Commands.nudge('activeTrack', 'attack' ) },
    { cap: 'U', label: 'decay',   commands: Commands.nudge('activeTrack', 'decay'  ) },
    { cap: 'I', label: 'sustain', commands: Commands.nudge('activeTrack', 'sustain') },
    { cap: 'O', label: 'release', commands: Commands.nudge('activeTrack', 'release') },
    { cap: 'P', label: 'cutoff',  commands: Commands.nudge('activeTrack', 'cutoff' ) }
  ),
  Commands.cap('N', {
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
    <svg xmlns='http://www.w3.org/2000/svg'>
      <path d={commands()} stroke-width='2'></path>
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>HOLD</b> mode controls the per-track <a href='https://en.wikipedia.org/wiki/Envelope_(music)#ADSR'>ADSR envelope</a>.
      In addition to the four standard stage controls, you can use <b>cutoff</b> to select how much the envelope affects the low-pass filter; and <b>sample</b> which toggles whether the envelope affects the sample oscillator.
    </Block>
  </>
);
