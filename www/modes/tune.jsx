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
