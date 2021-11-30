import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'G';

const subtabs = (effect) => Commands.tabbed(
  { cap: 'N', label: 'gain',  commands: Commands.nudge('song', `${effect}Gain` ) },
  { cap: 'M', label: 'feed',  commands: Commands.nudge('song', `${effect}Feed` ) },
  { cap: ',', label: 'space', commands: Commands.nudge('song', `${effect}Space`) }
);

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'reverb', commands: subtabs('reverb') },
  { cap: 'U', label: 'echo',   commands: subtabs('echo') },
  { cap: 'I', label: 'drive',  commands: subtabs('drive') }
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
  <svg xmlns='http://www.w3.org/2000/svg'>
    <For each={['reverb', 'echo', 'drive']}>
      {(effect, i) => <Fader effect={effect} i={i()} {...props} />}
    </For>
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
    </Block>
  </>
);
