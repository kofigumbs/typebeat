import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'G';

const subtabs = (effect) => Actions.tabbed(
  { cap: 'N', label: 'gain',  actions: Actions.nudge('song', `${effect}Gain` ) },
  { cap: 'M', label: 'feed',  actions: Actions.nudge('song', `${effect}Feed` ) },
  { cap: ',', label: 'space', actions: Actions.nudge('song', `${effect}Space`) }
);

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'reverb', actions: subtabs('reverb') },
  { cap: 'U', label: 'echo',   actions: subtabs('echo') },
  { cap: 'I', label: 'drive',  actions: subtabs('drive') }
);

const Fader = props => {
  const margin = 3;
  const x = (props.i+1) * 24;
  const y = createMemo(() => {
    const gain = props.state.song[`${props.effect}Gain`];
    const feed = props.state.song[`${props.effect}Feed`];
    const space = props.state.song[`${props.effect}Space`];
    return (1 - gain/50*(feed/100 + space/100)) * 40;
  });
  return (
    <>
      <path d={`M ${x} ${margin} v 40`} stroke-width='2' />
      <path d={`M ${x-6} ${y() + margin} h 12`} stroke-width='2' />
    </>
  );
};

export const Visual = props => (
  <svg xmlns="http://www.w3.org/2000/svg">
    <For each={['reverb', 'echo', 'drive']}>
      {(effect, i) => <Fader effect={effect} i={i()} {...props} />}
    </For>
  </svg>
);
