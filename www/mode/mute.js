import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => await proxy[`muted ${i}`] ? '</>' : '==',
    onDown: () => set('muted', i),
  })),
]);

export const visual = () => {};
export const sync = () => {};
