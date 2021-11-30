import Commands from '../commands';

export const cap = 'R';

export const commands = Commands.combine(
  Commands.cap('J', { label: () => 'oct. -', onDown: state => state.send('octave', 1) }),
  Commands.cap('K', { label: state => state.activeTrack.octave, title: () => true }),
  Commands.cap('L', { label: () => 'oct. +', onDown: state => state.send('octave', 2) }),
  Commands.cap('N', {
    label: () => 'use key',
    title: state => state.activeTrack.usingKey,
    onDown: state => state.send('usingKey', 0),
  }),
);

const Octave = props => {
  const h = 12;
  const x = props.i*14;
  const y = 40*(1-props.i/7);
  return (
    <>
      <path d={`M ${x} ${y} h ${h}`} stroke-width='2' />
      <Show when={props.state.activeTrack.octave === props.i + 2}>
        <circle cx={x+h/2} cy={y} r='4' stroke-width='2' className='transparent' />
      </Show>
    </>
  );
};

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <For each={Array.from({ length: 7 })}>{(_, i) => <Octave i={i()} {...props} />}</For>
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>TUNE</b> mode is another simple one (so far).
      It lets you adjust the octave range available in <b>NOTE</b> mode.
      The <b>use key</b> toggle determines whether the <b>SONG</b> mode <b>root</b>/<b>scale</b> apply to the active track.
    </Block>
  </>
);
