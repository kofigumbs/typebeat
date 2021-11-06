import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'W';

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'source', actions: Actions.tabbed(
    { cap: 'N', label: 'type', actions: Actions.select(
      'HJKL;',
      'activeTrack',
      'sampleType',
      ['file', 'live ->', 'live .=', 'live |>']
    )},
    { cap: 'M', label: 'level', actions: Actions.nudge('activeTrack', 'sampleLevel') },
    { cap: ',', label: 'detune', actions: Actions.nudge('activeTrack', 'sampleDetune') },
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
    <svg xmlns="http://www.w3.org/2000/svg">
      <For each={Array.from({ length: 24 })}>
        {(_, i) => <Waveform i={i()} {...props} />}
      </For>
    </svg>
  );
};
