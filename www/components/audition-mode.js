import Actions from '../actions';

export const cap = undefined;

export const actions = Actions.all({
  onDown: (state, i) => state.send('auditionDown', i),
  onUp: (state, i) => state.send('auditionUp', i),
});
