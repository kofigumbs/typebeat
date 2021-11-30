import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'W';

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'source', commands: Commands.tabbed(
    { cap: 'N', label: 'type', commands: Commands.select(
      'activeTrack',
      'sampleType',
      ['file', 'live ->', 'live .=', 'live |>']
    )},
    { cap: 'M', label: 'level', commands: Commands.nudge('activeTrack', 'sampleLevel') },
    { cap: ',', label: 'detune', commands: Commands.nudge('activeTrack', 'sampleDetune') },
  )},
);

const Waveform = props => {
  const s = createMemo(() => props.state.activeTrack[`waveform${props.i}`]/5 + 1);
  return (
    <path d={`M ${props.i*4 + 3} ${23 - s()} v ${s()*2}`} stroke-width='2' />
  );
};

export const Visual = props => {
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <For each={Array.from({ length: 24 })}>
        {(_, i) => <Waveform i={i()} {...props} />}
      </For>
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
    </Block>
  </>
);
