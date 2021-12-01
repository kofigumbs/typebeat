import { createSignal } from 'solid-js';

/*
 * DSL for defining the command map for each mode
 */

const Commands = {};

const none = () => '';

Commands.all = ({ label = none, title = none, onDown = none, onUp = none }) => {
  return new Map(Array.from('NM,./HJKL;YUIOP').map((cap, i) => [cap, {
    label:  (state) => label(state, i, cap),
    title:  (state) => title(state, i, cap),
    onDown: (state) => onDown(state, i, cap),
    onUp:   (state) => onUp(state, i, cap),
  }]));
};

Commands.cap = (cap, { label = none, title = none, onDown = none, onUp = none }) => {
  return new Map([[cap, { label, title, onDown, onUp }]]);
};

Commands.combine = (...commands) => {
  let entries = [];
  for (let command of commands) {
    entries = [...entries, ...command];
  }
  return new Map(entries);
};

Commands.tabbed = (...tabs) => {
  const [selected, setSelected] = createSignal(0);
  const selectCommands = new Map(tabs.map((tab, i) => [tab.cap, {
    label: () => tab.label,
    title: () => i === selected(),
    onDown: () => setSelected(i),
    onUp: none,
  }]));
  for (let tab of tabs) {
    tab.commands = Commands.combine(tab.commands, selectCommands);
  }
  return Commands.all({
    label:  (state, _, cap) => tabs[selected()].commands.get(cap)?.label(state),
    title:  (state, _, cap) => tabs[selected()].commands.get(cap)?.title(state),
    onDown: (state, _, cap) => tabs[selected()].commands.get(cap)?.onDown(state),
    onUp:   (state, _, cap) => tabs[selected()].commands.get(cap)?.onUp(state),
  });
};

Commands.select = (subject, method, labels) => Commands.combine(...labels.map((label, i) => (
  Commands.cap('HJKL;'[i], {
    label: () => label,
    title: (state) => i === state[subject][method],
    onDown: state => state.send(method, i),
  })
)));

Commands.nudge = (subject, method, one = 1, step = 10, format = (x => x)) => Commands.combine(
  Commands.cap('H', { label: () => `-${step}`, onDown: (state) => state.send(method, 0) }),
  Commands.cap('J', { label: () => `-${one}`,  onDown: (state) => state.send(method, 1) }),
  Commands.cap('L', { label: () => `+${one}`,  onDown: (state) => state.send(method, 2) }),
  Commands.cap(';', { label: () => `+${step}`, onDown: (state) => state.send(method, 3) }),
  Commands.cap('K', { label: (state) => format(state[subject][method]), title: () => true })
);

Commands.comingSoon = Commands.combine(
  Commands.cap('N', { label: () => 'coming' }),
  Commands.cap('M', { label: () => 'soon...' }),
);

export default Commands;
