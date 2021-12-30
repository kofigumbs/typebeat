import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'G';

const subtabs = (effect) => Commands.tabbed(
  { cap: 'N', label: 'gain', commands: Commands.nudge('song', `${effect}Gain`) },
  { cap: 'M', label: 'x',    commands: Commands.nudge('song', `${effect}X`) },
  { cap: ',', label: 'y',    commands: Commands.nudge('song', `${effect}Y`) }
);

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'echo',   commands: subtabs('echo') },
  { cap: 'U', label: 'reverb', commands: subtabs('reverb') },
  { cap: 'I', label: 'duck',   commands: subtabs('duck') }
);

const Fader = props => {
  const margin = 3;
  const x = (props.i+1) * 24;
  const y = createMemo(() => {
    const gain = props.state.song[`${props.effect}Gain`];
    const sendX = props.state.song[`${props.effect}X`];
    const sendY = props.state.song[`${props.effect}Y`];
    return (1 - gain/50*(sendX/100 + sendY/100)) * 40;
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
    <For each={['reverb', 'echo', 'duck']}>
      {(effect, i) => <Fader effect={effect} i={i()} {...props} />}
    </For>
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>RETURN</b> mode controls the characteristics of Typebeat's effects.
      Effects controls are song-wide, and each effect has independent <b>gain</b>, <b>x</b>, and <b>y</b> controls.
    </Block>
    <Block>
      Any values you change here will only affect the sound of tracks that have been routed to that effect.
      Use <b>SEND</b> mode to route individual track outputs to the various effects.
    </Block>
  </>
);
