import { createEffect, createSignal, on } from 'solid-js';

import Actions from '../actions';
import { note } from './note';

export const cap = 'Z';

const [taps, setTaps] = createSignal([]);

export const init = state => createEffect(on(
  () => state.modifier,
  () => setTaps([]),
  { defer: true }
));

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'tempo', actions: Actions.combine(
    Actions.cap('N', {
      label: () => 'play',
      title: (state) => state.song.playing,
      onDown: (state) => state.send('playing', 0),
    }),
    Actions.cap('M', {
      label: () => 'record',
      title: (state) => state.song.recording,
      onDown: (state) => state.send('recording', 0),
    }),
    Actions.cap(',', {
      label: () => 'tap',
      title: () => !!taps().length,
      onDown: (state) => {
        setTaps(taps => [...taps, performance.now()]);
        const t = taps();
        if (t.length === 1)
          return;
        let diffs = 0;
        for (let i = 1; i < t.length; i++)
          diffs += t[i] - t[i - 1];
        state.send('taps', Math.round(60000 / (diffs / (t.length - 1)) + 1));
      },
    }),
    Actions.nudge('song', 'tempo'),
  )},
  { cap: 'U', label: 'key', actions: Actions.combine(
    Actions.nudge('song', 'root', '1/2', '5th', root => note(root + 12)),
    Actions.select('NM,.', 'song', 'scale', ['major', 'minor', 'harm.', 'melodic'])
  )},
);

export const Visual = props => {
  const y = 23;
  const s = 12;
  const st = s*1.46;
  const cx = s+st + 6;
  const offset = (96-cx-s)/2;
  return (
    <svg xmlns="http://www.w3.org/2000/svg">
      <path
        d={`M ${offset} ${y-s} l ${st} ${s} l -${st} ${s} Z`}
        stroke-width="2"
        classList={{ secondary: props.state.song.playing }}
      />
      <circle
        r={s}
        cx={offset+cx}
        cy={y}
        stroke-width="2"
        classList={{ secondary: props.state.song.recording }}
      />
    </svg>
  );
};
