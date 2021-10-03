import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    onDown: () => set('auditionDown', i),
    onUp: () => set('auditionUp', i),
  })),
]);

export const visual = () => {};
export const sync = () => {};
