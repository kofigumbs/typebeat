import Actions from '../actions';

export const cap = 'C';

const subtabs = (effect) => Actions.tabbed(
  { cap: 'N', label: 'gain',  actions: Actions.nudge('song', `${effect}Gain` ) },
  { cap: 'M', label: 'feed',  actions: Actions.nudge('song', `${effect}Feed` ) },
  { cap: ',', label: 'space', actions: Actions.nudge('song', `${effect}Space`) }
);

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'reverb', actions: subtabs('reverb') },
  { cap: 'U', label: 'echo',   actions: subtabs('echo') },
  { cap: 'I', label: 'drive',  actions: subtabs('drive') }
);
