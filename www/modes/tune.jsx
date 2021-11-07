import Actions from '../actions';

export const cap = 'R';

export const actions = Actions.combine(
  Actions.cap('J', { label: () => 'oct. -', onDown: state => state.send('octave', 1) }),
  Actions.cap('K', { label: state => state.activeTrack.octave, title: () => true }),
  Actions.cap('L', { label: () => 'oct. +', onDown: state => state.send('octave', 2) }),
  Actions.cap('N', {
    label: () => 'use key',
    title: state => state.activeTrack.useKey,
    onDown: state => state.send('useKey', 0),
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
