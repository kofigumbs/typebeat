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

export const actions = Actions.combine(
  Actions.tabbed(
    { cap: 'Y', label: 'tempo', actions: Actions.combine(
      Actions.nudge('song', 'tempo'),
      Actions.cap('K', {
        label: (state) => state.song.tempo,
        title: () => true,
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
    )},
    { cap: 'U',
      label: 'root',
      actions: Actions.nudge('song', 'root', '1/2', '5th', root => note(root + 12)),
    },
    {
      cap: 'I',
      label: 'scale',
      actions: Actions.select('song', 'scale', ['major', 'minor', 'harm.', 'melodic']),
    },
  ),
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
);

export const Visual = props => {
  const y = 23;
  const s = 12;
  const st = s*1.46;
  const cx = s+st + 6;
  const offset = (96-cx-s)/2;
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <path
        d={`M ${offset} ${y-s} l ${st} ${s} l -${st} ${s} Z`}
        stroke-width='2'
        classList={{ secondary: props.state.song.playing }}
      />
      <circle
        r={s}
        cx={offset+cx}
        cy={y}
        stroke-width='2'
        classList={{ secondary: props.state.song.recording }}
      />
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>SONG</b> mode houses global playback controls.
      {' '}<b>SONG</b> mode has 3 value controls: <b>tempo</b> controls the track speed, while <b>root</b> and <b>scale</b> control the global pitch shift.
      The <b>play</b> and <b>record</b> commands are always available for quick access in the bottom row.
    </Block>
    <Block className='bullet'>
      Try pressing <kbd>N</kbd> to play/pause the demo baseline.
    </Block>
    <Block className='bullet'>
      Try recording additional layers to the demo beat -- press <kbd>M</kbd> to enable/disable recording.
      When <b>record</b> is on, any key you press in <b>AUDITION</b> mode will be recorded.
      Press <kbd>Z</kbd> from <b>SONG</b> mode to go back to <b>AUDITION</b> mode.
    </Block>
    <Block className='bullet'>
      Once you've tried playing and recording, press <kbd>A</kbd> to learn about <b>LOOP</b> mode.
    </Block>
  </>
);
