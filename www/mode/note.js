import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => bind.note(await proxy[`note ${i}`]),
    title: async () => i == await proxy.activeKey,
    onDown: () => set('noteDown', i),
    onUp: () => set('noteUp', i),
  })),
]);

export const visual = ({ el }) => {
};

export const sync = ({ el, local, proxy }) => {
};
