import Actions from '../actions';

export const cap = 'S';

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
