import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'G';

const subtabs = (effect, ...controls) => Commands.tabbed(
  ...controls.map((method, i) => ({
    cap: 'NM,'[i],
    label: method.toLowerCase(),
    commands: Commands.nudge('song', `${effect}${method}`),
  }))
);

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'echo',   commands: subtabs('echo', 'Gain', 'Length', 'Feed') },
  { cap: 'U', label: 'reverb', commands: subtabs('reverb', 'Gain', 'Size') },
  { cap: 'I', label: 'duck',   commands: subtabs('duck', 'Release') }
);

const Fader = props => {
  const margin = 3;
  const x = (props.i+1) * 24;
  return (
    <>
      <path d={`M ${x} ${margin} v 40`} stroke-width='2' />
      <path d={`M ${x-6} ${(1 - props.level)*40 + margin} h 12`} stroke-width='2' />
    </>
  );
};

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <Fader i={0} level={
      props.state.song.echoGain/50
        * (props.state.song.echoLength/100 + props.state.song.echoFeed/100)
    } />
    <Fader i={1} level={
      props.state.song.reverbGain/50 * props.state.song.reverbSize/50
    } />
    <Fader i={2} level={
      props.state.song.duckRelease/50
    } />
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>RETURN</b> mode controls the characteristics of Typebeat's effects.
      Effects controls are song-wide, and each effect has independent sub-controls that affect its sound.
    </Block>
    <Block>
      Any values you change here will only affect the sound of tracks that have been routed to that effect.
      Use <b>SEND</b> mode to route individual track outputs to the various effects.
    </Block>
  </>
);
