import Actions from '../actions';

export const cap = 'R';

export const actions = Actions.combine(
  Actions.cap('Y', {
    label: () => 'use key',
    title: state => state.activeTrack.useKey,
    onDown: state => state.send('useKey', 0),
  }),
  Actions.cap('J', { label: () => 'oct. -', onDown: state => state.send('octave', 1) }),
  Actions.cap('K', { label: () => 'octave', title: () => true }),
  Actions.cap('L', { label: () => 'oct. +', onDown: state => state.send('octave', 2) }),
);
