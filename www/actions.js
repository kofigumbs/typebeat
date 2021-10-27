const Actions = {};

const none = () => '';

Actions.all = ({ label = none, title = none, onDown = none, onUp = none }) => {
  return new Map(Array.from('NM,./HJKL;YUIOP').map((cap, i) => [cap, {
    label:  (state) => label(state, i, cap),
    title:  (state) => title(state, i, cap),
    onDown: (state) => onDown(state, i, cap),
    onUp:   (state) => onUp(state, i, cap),
  }]));
};

Actions.cap = (cap, { label = none, title = none, onDown = none, onUp = none }) => {
  return new Map([[cap, { label, title, onDown, onUp }]]);
};

Actions.combine = (...actions) => {
  let entries = [];
  for (let action of actions) {
    entries = [...entries, ...action];
  }
  return new Map(entries);
};

Actions.tabbed = (...tabs) => {
  let selected = 0;
  const selectActions = new Map(tabs.map((tab, i) => [tab.cap, {
    label: () => tab.label,
    title: () => i === selected,
    onDown: () => selected = i,
    onUp: none,
  }]));
  for (let tab of tabs) {
    tab.actions = Actions.combine(tab.actions, selectActions);
  }
  return Actions.all({
    label:  (state, _, cap) => tabs[selected].actions.get(cap)?.label(state),
    title:  (state, _, cap) => tabs[selected].actions.get(cap)?.title(state),
    onDown: (state, _, cap) => tabs[selected].actions.get(cap)?.onDown(state),
    onUp:   (state, _, cap) => tabs[selected].actions.get(cap)?.onUp(state),
  });
};

Actions.nudge = (subject, method, step = 10) => Actions.combine(
  Actions.cap('H', { label: () => `-${step}`, onDown: (state) => state.send(method, 0) }),
  Actions.cap('J', { label: () => '-1',       onDown: (state) => state.send(method, 1) }),
  Actions.cap('L', { label: () => '+1',       onDown: (state) => state.send(method, 2) }),
  Actions.cap(';', { label: () => `+${step}`, onDown: (state) => state.send(method, 3) }),
  Actions.cap('K', { label: (state) => state[subject][method], title: () => true })
);

Actions.comingSoon = Actions.combine(
  Actions.cap('N', { label: () => 'coming' }),
  Actions.cap('M', { label: () => 'soon...' }),
);

Actions.note = n => {
  const name = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'][n % 12];
  const octave = Math.floor(n / 12 - 1);
  return `${name}${octave}`;
};

export default Actions;
