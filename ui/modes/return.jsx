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
  { cap: 'U', label: 'reverb', commands: subtabs('reverb', 'Gain', 'Comb', 'Damp') },
  { cap: 'I', label: 'duck',   commands: subtabs('duck', 'Release') }
);

const Fader = props => {
  const margin = 3;
  const x = (props.x+1) * 24;
  const y = createMemo(() => {
  });
  return (
    <>
      <path d={`M ${x} ${margin} v 40`} stroke-width='2' />
      <path d={`M ${x-6} ${(1 - props.value)*40 + margin} h 12`} stroke-width='2' />
    </>
  );
};

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <Fader x={0} value={
      props.state.song.reverbGain/50
        * (props.state.song.reverbComb/100 + props.state.song.reverbDamp/100)
    } />
    <Fader x={1} value={
      props.state.song.echoGain/50
        * (props.state.song.echoLength/100 + props.state.song.echoFeed/100)
    } />
    <Fader x={2} value={
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
