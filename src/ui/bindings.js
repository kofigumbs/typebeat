export const modes = new Map([
  ['Q', 'Track'], ['W', 'Sound'], ['E', 'Chop'], ['R', 'Poly'], ['T', 'Note'],
  ['A', 'Beat'],  ['S', 'Loop'],  ['D', 'Hold'], ['F', 'EQ'],   ['G', 'Mix'],
  ['Z', 'Key'],   ['X', 'Auto'],  ['C', 'Send'], ['V', 'Tape'], ['B', 'Mute'],
  [undefined, 'Audition'],
]);

export const bindActions = (local, proxy, set) => {
  const join = (a, b) => `${a}${b[0].toUpperCase()}${b.substring(1)}`.replace(/[^\w]/, '');

  const noOp = () => '';
  const bind = options => Object.assign({ label: noOp, title: noOp, onDown: noOp, onUp: noOp }, options);

  const title = label => bind({ label, title: () => true });
  const toggle = (label, title, onDown) => bind({ label: () => label, title, onDown });

  const group = (caps, f) => Array.from(caps, (cap, i) => [cap, bind(f(i))]);
  const oneOf = (caps, name, labels) => {
    local[name] = labels[0];
    return group(caps, i => ({
      label: () => labels[i],
      title: () => local[name] === labels[i],
      onDown: () => local[name] = labels[i],
    }));
  };

  const all = f => group('NM,./HJKL;YUIOP', f);
  const nudge = (value, onDown) => [
    ['H', bind({ label: () => '-10', onDown: () => onDown(0) }) ],
    ['J', bind({ label: () => '-1',  onDown: () => onDown(1) }) ],
    ['K', title(value) ],
    ['L', bind({ label: () => '+1',  onDown: () => onDown(2) }) ],
    [';', bind({ label: () => '+10', onDown: () => onDown(3) }) ],
  ];

  const note = n => {
    const name = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'][n % 12];
    const octave = Math.floor(n / 12 - 1);
    return `${name}${octave}`;
  };

  return new Map([
    ['Track', new Map([
      ...all(i => ({
        label: async () => i === await proxy.activeTrack ? 'active' : '',
        title: async () => i === await proxy.activeTrack,
        onDown: () => set('activeTrack', i),
      })),
    ])],
    ['Sound', new Map([
      ...oneOf('YUIO', 'sound', ['sample', 'synth 1', 'synth 2', 'synth 3']),
      ...oneOf('NM,', 'soundControl', ['type', 'level', 'detune']),
      ...group('HJKL;', i => {
        const soundMethod = () => join(local.sound, local.soundControl);
        const soundNudge = nudge(() => proxy[soundMethod()], j => set(soundMethod(), j))[i][1];
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
    ])],
    ['Chop', new Map([
    ])],
    ['Poly', new Map([
      ['Y', toggle('use key', () => proxy.useKey, () => set('useKey')) ],
      ['J', bind({ label: () => 'oct. -', onDown: () => set('octave', 1) }) ],
      ['K', title(() => proxy.octave) ],
      ['L', bind({ label: () => 'oct. +', onDown: () => set('octave', 2) }) ],
    ])],
    ['Note', new Map([
      ...all(i => ({
        label: async () => note(await proxy[`note ${i}`]),
        title: async () => i == await proxy.activeKey,
        onDown: () => set('noteDown', i),
        onUp: () => set('noteUp', i),
      })),
    ])],
    ['Beat', new Map([
      ['Y', title(() => 'tempo')],
      ...nudge(() => proxy.tempo, i => set('tempo', i)),
      ['N', toggle('play', () => proxy.playing, () => set('playing')) ],
      ['M', toggle('arm', () => proxy.armed, () => set('armed')) ],
      [',', bind({
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
    ])],
    ['Loop', new Map([
      ...group('YUHJL;', i => ({
        label: () => ['bars -', 'bars +','zoom -', 'page -', 'page +', 'zoom +'][i],
        onDown: () => set(...[['bars', -1], ['bars', 1], ['zoomOut'], ['page', -1], ['page', 1], ['zoomIn']][i]),
      })),
      ...group('NM,.', i => ({
        label: async () => {
          const n = ((await proxy.viewStart + i) % await proxy.resolution) + 1;
          switch (await proxy[`view ${i}`]) {
            case 0: return '';
            case 1: return `${n}/${await proxy.resolution}`;
            case 2: return `${n}█${await proxy.resolution}`;
            case 3: return `${n}░${await proxy.resolution}`;
          }
        },
        onDown: () => set('sequence', i),
      })),
      ['P', bind({ label: () => 'clear', title: () => proxy.canClear, onDown: () => set('clear') }) ],
      ['K', title(async () => `bar ${((await proxy.viewStart / await proxy.resolution)|0) + 1}/${await proxy.bars}`) ],
    ])],
    ['Hold', new Map([
      ...oneOf('YUIOP', 'hold', ['attack', 'decay', 'sustain', 'release', 'cutoff']),
      ...nudge(async () => await proxy[local.hold], i => set(local.hold, i)),
      ['N', toggle('sample', async () => await proxy.holdSample, () => set('holdSample')) ],
    ])],
    ['EQ', new Map([
      ...oneOf('YUIOP', 'eqBand', ['low', 'band 1', 'band 2', 'band 3', 'high']),
      ...oneOf('NM', 'eqFilter', ['freq.', 'res.']),
      ...nudge(() => proxy[join(local.eqBand, local.eqFilter)], i => set(join(local.eqBand, local.eqFilter), i)),
    ])],
    ['Mix', new Map([
      ...oneOf('YUIOP', 'mix', ['main', 'pan', 'reverb', 'echo', 'drive']),
      ...nudge(async () => await proxy[local.mix], i => set(local.mix, i)),
    ])],
    ['Key', new Map([
      ['Y', title(() => 'root')],
      ['K', title(async () => note(await proxy.root + 12)) ],
      ...group('HJL;', i => ({
        label: () => ['-5th', '-1/2', '+1/2', '+5th'][i],
        onDown: () => set('root', i),
      })),
      ...group('NM,.', i => ({
        label: () => ['major', 'minor', 'harm.', 'melodic'][i],
        title: async () => i === await proxy.scale,
        onDown: () => set('scale', i),
      })),
    ])],
    ['Auto', new Map([
    ])],
    ['Send', new Map([
      ...oneOf('YUI', 'effect', ['reverb', 'echo', 'drive']),
      ...nudge(() => proxy[join(local.effect, local.effectControl)], i => set(join(local.effect, local.effectControl), i)),
      ...oneOf('NM,', 'effectControl', ['gain', 'feed', 'space']),
    ])],
    ['Tape', new Map([
    ])],
    ['Mute', new Map([
      ...all(i => ({
        label: async () => await proxy[`muted ${i}`] ? '</>' : '==',
        onDown: () => set('muted', i),
      })),
    ])],
    ['Audition', new Map([
      ...all(i => ({
        onDown: () => set('auditionDown', i),
        onUp: () => set('auditionUp', i),
      })),
    ])],
  ]);
};
