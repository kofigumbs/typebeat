import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'T';

export const actions = Actions.all({
  label: (state, i) => Actions.note(state.activeTrack[`note${i}`]),
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
  <svg xmlns="http://www.w3.org/2000/svg">
    <Keys state={props.state} component={White} offset={0} ids={[0, 2, 4, 5, 7, 9, 11]} />
    <Keys state={props.state} component={Black} offset={0} ids={[1, 3]} />
    <Keys state={props.state} component={Black} offset={3} ids={[6, 8, 10]} />
  </svg>
);
