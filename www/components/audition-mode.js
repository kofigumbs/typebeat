import bind from '../bind';

export const cap = undefined;

export const actions = (state) => new Map([
  ...bind.all(i => ({
    onDown: () => state.send('auditionDown', i),
    onUp: () => state.send('auditionUp', i),
  })),
]);
