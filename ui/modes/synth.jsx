import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'E';

export const subtab = i => Commands.tabbed(
  { cap: 'N', label: 'type', commands: Commands.select(
    'activeTrack',
    `synth${i}Type`,
    ['sine', 'tri.', 'saw', 'square', 'noise']
  )},
  { cap: 'M', label: 'level', commands: Commands.nudge('activeTrack', `synth${i}Level`) },
  { cap: ',', label: 'detune', commands: Commands.nudge('activeTrack', `synth${i}Detune`) },
);

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'osc. 1', commands: subtab(1) },
  { cap: 'U', label: 'osc. 2', commands: subtab(2) },
  { cap: 'I', label: 'osc. 3', commands: subtab(3) },
);

const length = 12;
const offsets = [
  (i) => ({ i, y: Math.sin(i/length*2*Math.PI) }),
  (i) => ({ i, y: [0, 1, 0, -1][i % 4] }),
  (i) => ({ i: i-Math.floor(i/4), y: [-1, -1/3, 1/3, 1][i % 4] }),
  (i) => ({ i: i-Math.floor(i/4), y: i % 8 < 4 ? 1 : -1 }),
  (i, noise) => ({ i, y: noise[i] }),
];

const Path = props => {
  const noise = Array.from({ length }, () => Math.random()*2 - 1);
  const commands = createMemo(() => {
    const commands = [];
    const type = props.state.activeTrack[`synth${props.osc}Type`];
    const level = props.state.activeTrack[`synth${props.osc}Level`];
    const detune = props.state.activeTrack[`synth${props.osc}Detune`];
    let offset = { i: 0 }; // saw and square waves need to adjust the counter itself
    for (let i = 0; offset.i < length; i++) {
      offset = offsets[type](i, noise);
      const x = 47 - 4*(offset.i - length/2) + detune/4;
      const y = 22 + 10*(props.osc-2) + level*offset.y/4;
      commands.push(`${x} ${y}`);
    }
    return commands;
  });
  return (
    <path d={`M ${commands().join(' L ')}`} stroke-width='2' className='transparent' />
  );
};

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <For each={[1, 2, 3]}>{osc => <Path osc={osc} {...props} />}</For>
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>SYNTH</b> mode controls the track's three synth oscillators.
      Each oscillator has independent <b>type</b>, <b>level</b>, and <b>detune</b> controls.
      By default, each oscillator is turned all the way down.
    </Block>
    <Block className='bullet'>
      Try using <b>TRACK</b> mode to select the <kbd>P</kbd> demo track, which is preset with some synth controls.
    </Block>
  </>
);
