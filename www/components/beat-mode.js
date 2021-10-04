import bind from '../bind';

export const cap = 'A';

export const actions = (local, proxy, set) => new Map([
  ['Y', bind.title(() => 'tempo')],
  ...bind.nudge(() => proxy.tempo, i => set('tempo', i)),
  ['N', bind.toggle('play', () => proxy.playing, () => set('playing')) ],
  ['M', bind.toggle('arm', () => proxy.armed, () => set('armed')) ],
  [',', bind.one({
    label: () => 'tap',
    title: () => !!local.tempoTaps.length,
    onDown: (time) => {
      local.tempoTaps.push(time);
      if (local.tempoTaps.length === 1)
        return;
      let diffs = 0;
      for (let i = 1; i < local.tempoTaps.length; i++)
        diffs += local.tempoTaps[i] - local.tempoTaps[i - 1];
      set('tempoTaps', Math.round(60000 / (diffs / (local.tempoTaps.length - 1)) + 1));
    },
  })],
]);
