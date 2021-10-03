import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ['Y', bind.title(() => 'root')],
  ['K', bind.title(async () => bind.note(await proxy.root + 12)) ],
  ...bind.group('HJL;', i => ({
    label: () => ['-5th', '-1/2', '+1/2', '+5th'][i],
    onDown: () => set('root', i),
  })),
  ...bind.group('NM,.', i => ({
    label: () => ['major', 'minor', 'harm.', 'melodic'][i],
    title: async () => i === await proxy.scale,
    onDown: () => set('scale', i),
  })),
]);

export const visual = () => {};
export const sync = () => {};
