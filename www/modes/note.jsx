import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'T';

export const note = n => {
  const name = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'][n % 12];
  const octave = Math.floor(n / 12 - 1);
  return `${name}${octave}`;
};

export const actions = Actions.all({
  label: (state, i) => note(state.activeTrack[`note${i}`]),
  title: (state, i) => i == state.activeTrack.activeKey,
  onDown: (state, i) => state.send('noteDown', i),
  onUp: (state, i) => state.send('noteUp', i),
});

const H = 49;
const W = 14;
const S = 10;

const Key = props => (
  <rect x={props.x-2} y='-2' width={props.width} height={props.height} stroke-width='2' classList={{
    secondary: props.id === props.state.activeTrack[`note${props.state.activeTrack.activeKey}`] % 12
  }} />
);

const Black = props => (
  <Key state={props.state} id={props.id} x={(props.x+1)*W - S/2} width={S} height={H/2} />
);

const White = props => (
  <Key state={props.state} id={props.id} x={props.x*W} width={W} height={H} />
);

const Keys = props => {
  let Component = props.component;
  return (
    <For each={props.ids}>
      {(id, i) => <Component state={props.state} id={id} x={i() + props.offset} />}
    </For>
  );
};

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <Keys state={props.state} component={White} offset={0} ids={[0, 2, 4, 5, 7, 9, 11]} />
    <Keys state={props.state} component={Black} offset={0} ids={[1, 3]} />
    <Keys state={props.state} component={Black} offset={3} ids={[6, 8, 10]} />
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      In <b>NOTE</b> mode, each key plays a different note on the active track.
      The lowest note is on the bottom-left, and the highest is on the top-right.
      The last note you use is the active note, which determines which note you are sequencing in <b>LOOP</b> mode.
    </Block>
    <Block className='bullet'>
      Try playing "mary had a little lamb" by rhythmically typing this sequence:
      {' '}<kbd>I</kbd><kbd>U</kbd><kbd>Y</kbd><kbd>U</kbd><kbd>I</kbd><kbd>I</kbd><kbd>I</kbd>.
    </Block>
  </>
);
