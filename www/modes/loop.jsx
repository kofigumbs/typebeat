import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'A';

const view = i => state => {
  const n = ((state.activeTrack.viewStart + i) % state.activeTrack.resolution) + 1;
  switch (state.activeTrack[`view${i}`]) {
    case 0: return '';
    case 1: return `${n}/${state.activeTrack.resolution}`;
    case 2: return `${n}█${state.activeTrack.resolution}`;
    case 3: return `${n}░${state.activeTrack.resolution}`;
  }
};

export const actions = Actions.combine(
  Actions.cap('Y', { label: () => 'bars -', onDown: state => state.send('length', -1) }),
  Actions.cap('U', { label: () => 'bars +', onDown: state => state.send('length', 1) }),
  Actions.cap('H', { label: () => 'zoom -', onDown: state => state.send('zoomOut', 0) }),
  Actions.cap('J', { label: () => 'page -', onDown: state => state.send('page', -1) }),
  Actions.cap('L', { label: () => 'page +', onDown: state => state.send('page', 1) }),
  Actions.cap(';', { label: () => 'zoom +', onDown: state => state.send('zoomIn', 0) }),
  Actions.cap('K', { title: () => true, label: state => {
    const bar = Math.floor(state.activeTrack.viewStart / state.activeTrack.resolution) + 1;
    return `bar ${bar}/${state.activeTrack.bars}`
  }}),
  Actions.cap('P', {
    label: () => 'clear',
    title: state => state.activeTrack.canClear,
    onDown: state => state.send('clear', 0),
  }),
  Actions.cap('N', { label: view(0), onDown: state => state.send('sequence', 0) }),
  Actions.cap('M', { label: view(1), onDown: state => state.send('sequence', 1) }),
  Actions.cap(',', { label: view(2), onDown: state => state.send('sequence', 2) }),
  Actions.cap('.', { label: view(3), onDown: state => state.send('sequence', 3) }),
);

const View = props => {
  const s = 16;
  const x = 16 + props.i*16;
  const y = 15;
  return (
    <>
      <rect x={x} y={y} width={s} height={s} stroke-width='2' />
      <Show when={props.state.activeTrack[`view${props.i}`] > 1}>
        <rect x={x+4} y={y+4} width={s-8} height={s-8} className='dark' stroke-width='2' />
      </Show>
    </>
  );
};

export const Visual = props => {
  const markStart = createMemo(() => {
    const position = (props.state.song.step / props.state.activeTrack.length) % 1;
    const pageLength = props.state.activeTrack.resolution / 4;
    return Math.floor(position * pageLength) / pageLength;
  });
  const markLength = createMemo(() => {
    return 4 / props.state.activeTrack.resolution / props.state.activeTrack.bars;
  });
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <path d='M 3 43 h 90' stroke-width='2' />
      <path d={`M ${3 + 90*markStart()} 39 h ${90*markLength()}`} stroke-width='2' />
      <For each={[0, 1, 2, 3]}>
        {i => <View i={i} {...props} />}
      </For>
    </svg>
  );
};
