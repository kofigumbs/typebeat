import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIO', 'sound', ['sample', 'synth 1', 'synth 2', 'synth 3'], local),
  ...bind.oneOf('NM,', 'soundControl', ['type', 'level', 'detune'], local),
  ...bind.group('HJKL;', i => {
    const soundMethod = () => join(local.sound, local.soundControl);
    const soundNudge = bind.nudge(() => proxy[soundMethod()], j => set(soundMethod(), j))[i][1];
    return {
      label: () => {
        if (local.soundControl !== 'type')
          return soundNudge.label();
        else if (local.sound === 'sample')
          return ['file', 'live ->', 'live .=', 'live |>'][i]
        else
          return ['sine', 'tri.', 'saw', 'square', 'noise'][i];
      },
      title: async () => (
        local.soundControl === 'type' ? i === await proxy[soundMethod()] : soundNudge.title()
      ),
      onDown: async () => {
        local.soundControl === 'type' ? set(soundMethod(), i) : soundNudge.onDown();
      },
    };
  }),
]);

export const visual = ({ el }) => {
};

export const sync = ({ el, local, proxy }) => {
};
