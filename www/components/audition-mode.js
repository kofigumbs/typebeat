import bind from '../bind';

export const cap = undefined;

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    onDown: () => set('auditionDown', i),
    onUp: () => set('auditionUp', i),
  })),
]);
