import Actions from '../actions';
import { note } from './note';

export const cap = 'Z';

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'root', actions: Actions.combine(
    Actions.nudge('song', 'root', '1/2', '5th', root => note(root + 12)),
    Actions.select('NM,.', 'song', 'scale', ['major', 'minor', 'harm.', 'melodic'])
  )},
);
