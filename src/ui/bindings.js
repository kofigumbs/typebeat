const Bindings = ({ state, set }) => {
  const method = (...parts) => parts.join(' ');

  const noOp = () => '';
  const bind = options => Object.assign({ label: noOp, title: noOp, onDown: noOp, onUp: noOp }, options);

  const title = label => bind({ label, title: () => true });
  const toggle = (label, title, onDown) => bind({ label: () => label, title, onDown });

  const group = (caps, f) => Array.from(caps, (cap, i) => [cap, bind(f(i))]);
  const oneOf = (caps, state, name, labels) => {
    state[name] = labels[0];
    return group(caps, i => ({
      label: () => labels[i],
      title: () => state[name] === labels[i],
      onDown: () => state[name] = labels[i],
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
    ['Q', { actions: new Map([
      ...all(i => ({
        label: async () => i === await state.activeTrack ? 'active' : '',
        title: async () => i === await state.activeTrack,
        onDown: () => set('activeTrack', i),
      })),
    ])}],
    ['W', { mode: 'Sound', actions: new Map([
      ...oneOf('YUIO', state, 'sound', ['sample', 'synth 1', 'synth 2', 'synth 3']),
      ...oneOf('NM,', state, 'soundControl', ['type', 'level', 'detune']),
      ...group('HJKL;', i => {
        const soundMethod = () => method(state.sound, state.soundControl);
        const soundNudge = nudge(() => state[soundMethod()], j => set(soundMethod(), j))[i][1];
        return {
          label: () => {
            if (state.soundControl !== 'type')
              return soundNudge.label();
            else if (state.sound === 'sample')
              return ['file', 'live ->', 'live .=', 'live |>'][i]
            else
              return ['sine', 'tri.', 'saw', 'square', 'noise'][i];
          },
          title: async () => (
            state.soundControl === 'type' ? i === await state[soundMethod()] : soundNudge.title()
          ),
          onDown: async () => {
            state.soundControl === 'type' ? set(soundMethod(), i) : soundNudge.onDown();
          },
        };
      }),
    ])}],
    ['E', { mode: 'Chop', actions: new Map([
    ])}],
    ['R', { mode: 'Poly', actions: new Map([
      ['Y', toggle('use key', () => state.useKey, () => set('useKey')) ],
      ['J', bind({ label: () => 'oct. -', onDown: () => set('octave', 1) }) ],
      ['K', title(() => state.octave) ],
      ['L', bind({ label: () => 'oct. +', onDown: () => set('octave', 2) }) ],
    ])}],
    ['T', { mode: 'Note', actions: new Map([
      ...all(i => ({
        label: async () => note(await state[method('note', i)]),
        title: async () => i == await state.lastKey,
        onDown: () => set('noteDown', i),
        onUp: () => set('noteUp', i),
      })),
    ])}],
    ['A', { mode: 'Beat', actions: new Map([
      ['Y', title(() => 'tempo')],
      ...nudge(() => state.tempo, i => set('tempo', i)),
      ['N', toggle('play', () => state.playing, () => set('play')) ],
      ['M', toggle('arm', () => state.armed, () => set('arm')) ],
      [',', bind({
        label: () => 'tap',
        title: () => !!state.tempoTaps.length,
        onDown: (time) => {
          state.tempoTaps.push(time);
          if (state.tempoTaps.length === 1)
            return;
          let diffs = 0;
          for (let i = 1; i < state.tempoTaps.length; i++)
            diffs += state.tempoTaps[i] - state.tempoTaps[i - 1];
          set('tempoTaps', Math.round(60000 / (diffs / (state.tempoTaps.length - 1)) + 1));
        },
      })],
    ])}],
    ['S', { mode: 'Loop', actions: new Map([
      ...group('YUHJL;', i => ({
        label: () => ['bars -', 'bars +','zoom -', 'page -', 'page +', 'zoom +'][i],
        onDown: () => set(...[['bars', -1], ['bars', 1], ['zoomOut'], ['page', -1], ['page', 1], ['zoomIn']][i]),
      })),
      ...group('NM,.', i => ({
        label: async () => {
          const n = ((await state.viewStart + i) % await state.resolution) + 1;
          switch (await state[method('view', i)]) {
            case 0: return '';
            case 1: return `${n}/${await state.resolution}`;
            case 2: return `${n}█${await state.resolution}`;
            case 3: return `${n}░${await state.resolution}`;
          }
        },
        onDown: () => set('sequence', i),
      })),
      ['P', bind({ label: () => 'clear', title: () => state.canClear, onDown: () => set('clear') }) ],
      ['K', title(async () => `bar ${((await state.viewStart / await state.resolution)|0) + 1}/${await state.bars}`) ],
    ])}],
    ['D', { mode: 'Hold', actions: new Map([
      ...oneOf('YUIOP', state, 'hold', ['attack', 'decay', 'sustain', 'release', 'cutoff']),
      ...nudge(async () => await state[state.hold], i => set(state.hold, i)),
      ['N', toggle('sample', async () => await state.holdSample, () => set('holdSample')) ],
    ])}],
    ['F', { mode: 'EQ', actions: new Map([
      ...oneOf('YUIOP', state, 'eqBand', ['low', 'band 1', 'band 2', 'band 3', 'high']),
      ...oneOf('NM', state, 'eqFilter', ['freq.', 'res.']),
      ...nudge(() => state[method(state.eqBand, state.eqFilter)], i => set(method(state.eqBand, state.eqFilter), i)),
    ])}],
    ['G', { mode: 'Mix', actions: new Map([
      ...oneOf('YUIOP', state, 'mix', ['main', 'pan', 'reverb', 'echo', 'drive']),
      ...nudge(async () => await state[state.mix], i => set(state.mix, i)),
    ])}],
    ['Z', { mode: 'Key', actions: new Map([
      ['Y', title(() => 'root')],
      ['K', title(async () => note(await state.root + 12)) ],
      ...group('HJL;', i => ({
        label: () => ['-5th', '-1/2', '+1/2', '+5th'][i],
        onDown: () => set('root', i),
      })),
      ...group('NM,.', i => ({
        label: () => ['major', 'minor', 'harm.', 'melodic'][i],
        title: async () => i === await state.scale,
        onDown: () => set('scale', i),
      })),
    ])}],
    ['X', { mode: 'Auto', actions: new Map([
    ])}],
    ['C', { mode: 'Send', actions: new Map([
      ...oneOf('YUI', state, 'effect', ['reverb', 'echo', 'drive']),
      ...nudge(() => state[method(state.effect, state.effectControl)], i => set(method(state.effect, state.effectControl), i)),
      ...oneOf('NM,', state, 'effectControl', ['gain', 'feed', 'space']),
    ])}],
    ['V', { mode: 'Tape', actions: new Map([
    ])}],
    ['B', { mode: 'Mute', actions: new Map([
      ...all(i => ({
        label: async () => await state[method('mute', i)] ? '</>' : '==',
        onDown: () => set('mute', i),
      })),
    ])}],
    [undefined, { actions: new Map([
      ...all(i => ({
        onDown: () => set('auditionDown', i),
        onUp: () => set('auditionUp', i),
      })),
    ])}],
  ]);
};
