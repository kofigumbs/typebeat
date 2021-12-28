import { createMemo } from 'solid-js';

import Commands from '../commands';

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

export const commands = Commands.combine(
  Commands.cap('Y', { label: () => 'bars -', onDown: state => state.send('length', -1) }),
  Commands.cap('U', { label: () => 'bars +', onDown: state => state.send('length', 1) }),
  Commands.cap('H', { label: () => 'zoom -', onDown: state => state.send('zoomOut', 0) }),
  Commands.cap('J', { label: () => 'page -', onDown: state => state.send('page', -1) }),
  Commands.cap('L', { label: () => 'page +', onDown: state => state.send('page', 1) }),
  Commands.cap(';', { label: () => 'zoom +', onDown: state => state.send('zoomIn', 0) }),
  Commands.cap('K', { title: () => true, label: state => {
    const bar = Math.floor(state.activeTrack.viewStart / state.activeTrack.resolution) + 1;
    return `bar ${bar}/${state.activeTrack.bars}`
  }}),
  Commands.cap('P', {
    label: () => 'clear',
    title: state => state.activeTrack.canClear,
    onDown: state => state.send('clear', 0),
  }),
  Commands.cap('N', { label: view(0), onDown: state => state.send('sequence', 0) }),
  Commands.cap('M', { label: view(1), onDown: state => state.send('sequence', 1) }),
  Commands.cap(',', { label: view(2), onDown: state => state.send('sequence', 2) }),
  Commands.cap('.', { label: view(3), onDown: state => state.send('sequence', 3) }),
);

const View = props => {
  const s = 16;
  const x = 16 + props.i*16;
  const y = 15;
  const active = createMemo(() => {
    const step = props.state.song.step % props.state.activeTrack.length;
    const index = props.state.activeTrack[`viewIndex${props.i}`];
    const length = props.state.activeTrack.viewLength;
    return props.state.song.playing && step >= index && step < index + length;
  });
  return (
    <>
      <rect x={x} y={y} width={s} height={s} stroke-width='2' classList={{ accent: active() }} />
      <Show when={props.state.activeTrack[`view${props.i}`] > 1}>
        <rect x={x+4} y={y+4} width={s-8} height={s-8} className='background' stroke-width='2' />
      </Show>
    </>
  );
};

export const Visual = props => {
  const markStart = createMemo(() => {
    const position = (props.state.song.step / props.state.activeTrack.length) % 1;
    const pageLength = props.state.activeTrack.resolution * props.state.activeTrack.bars / 4;
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

export const Help = ({ Block }) => (
  <>
    <Block>
      Every track starts with its own 16-step loop.
      In <b>LOOP</b> mode, you can view 4 steps at a time.
      You can use <b>page -</b> and <b>page +</b> to move your view across the 16 steps in your loop.
    </Block>
    <Block>
      Keys on the bottom row represent the steps themselves.
      If the key is marked with █, then that step will trigger a sound.
      Pressing any of those keys will toggle the trigger on that step, which makes it an alternative to recording your sequence live.
    </Block>
    <Block className='bullet'>
      Try manually entering (or editing) steps in the loop.
      If you keep the song playing, then you can hear your changes as you go.
    </Block>
    <Block className='bullet'>
      Try zooming in and out.
      Each track starts with a 16th note resolution, but you can zoom in up to 512th notes.
      The zoom level also corresponds to the <a href="https://en.wikipedia.org/wiki/Quantization_(music)">quantization</a> when recording live.
    </Block>
    <Block className='bullet'>
      Each track has its own loop, so try using <b>LOOP</b> mode on other tracks.
      Press <kbd>Q</kbd> to use <b>TRACK</b> mode and change tracks.
    </Block>
  </>
);
