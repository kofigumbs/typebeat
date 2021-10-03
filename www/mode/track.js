import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => i === await proxy.activeTrack ? 'active' : '',
    title: async () => !await proxy.playing,
    onDown: () => set('activeTrack', i),
  })),
]);

export const visual = ({ el }) => {
};

export const sync = ({ el, local, proxy }) => {
};
