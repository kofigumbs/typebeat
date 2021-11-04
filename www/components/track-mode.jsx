import { createRenderEffect, on } from 'solid-js';

import Actions from '../actions';
import { pulse } from '../animations';

export const cap = 'Q';

export const actions = Actions.all({
  label: (state, i) => i === state.song.activeTrack ? 'active' : '',
  title: (state) => !state.song.playing,
  onDown: (state, i) => state.send('activeTrack', i),
  onUp: (state, i) => state.send('auditionUp', i),
});

export const layout = (x, y) => {
  const S = 16;
  const DX = 9;
  return { width: S, height: S, x: S*x + DX*y - 1, y: S*y - 1, 'stroke-width': 2 };
};


const Track = props => {
  let ref;
  createRenderEffect(on(
    () => props.state.tracks[props.id].recent,
    () => pulse(ref),
    { defer: true }
  ));
  return (
    <rect ref={ref} {...layout(props.x, props.y)} style={{
      '--key_fill': props.id === props.state.song.activeTrack ? 'var(--secondary)' : 'var(--key_background)',
      '--key_pulse': 'transparent',
    }} />
  );
};

const Row = props => {
  const Component = props.component;
  return (
    <For each={props.tracks}>
      {(id, x) => <Component id={id} x={x()} y={props.y} {...props} />}
    </For>
  );
};

export const Grid = props => (
  <svg xmlns="http://www.w3.org/2000/svg">
    <rect width='100%' height='100%' style={{ fill: 'var(--dark)' }} />
    <Row y={0} tracks={[10, 11, 12, 13, 14]} {...props} />
    <Row y={1} tracks={[ 5,  6,  7,  8,  9]} {...props} />
    <Row y={2} tracks={[ 0,  1,  2,  3,  4]} {...props} />
  </svg>
);

export const Visual = props => (
  <Grid component={Track} {...props} />
);
