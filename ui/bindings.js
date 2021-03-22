const Bindings = ({ state, send }) => {
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
  const fill = ['/', toggle('FILL', async () => await state.fill, () => {}) ];
  const nudge = (value, onDown) => [
    ['K', title(value) ],
    ...group('HJL;', i => ({ label: () => ['-10', -1, 1, '+10'][i], onDown: () => onDown(i) })),
  ];

  return new Map([
    ['Q', { actions: new Map([
      ...all(i => ({
        label: async () => i === await state.activeVoice ? 'active' : '',
        title: async () => i === await state.activeVoice,
        onDown: () => send('activateVoice', i),
      })),
    ])}],
    ['W', { mode: 'Source', actions: new Map([
      ['Y', toggle('sample', async () => await state.source === 0, () => send('source', 0)) ],
      ['U', toggle('input', async () => await state.source === 1, () => send('source', 1)) ],
    ])}],
    ['E', { mode: 'Chop', actions: new Map([
    ])}],
    ['R', { mode: 'Poly', actions: new Map([
    ])}],
    ['T', { mode: 'Note', actions: new Map([
      ...all(i => ({
        label: async () => {
          const note = await state[`note:${i}`];
          const name = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][note % 12];
          const octave = Math.floor(note / 12 - 1);
          return `${name}${octave}`;
        },
        title: async () => await state[`note:${i}`] === await state.naturalNote,
        onDown: () => send('noteDown', i),
        onUp: () => send('noteUp', i),
      })),
    ])}],
    ['A', { mode: 'Loop', actions: new Map([
      ...group('HJL;', i => ({
        label: () => ['zoom -', 'page -', 'page +', 'zoom +'][i],
        onDown: () => send('view', i),
      })),
      ...group('NM,.', i => ({
        label: async () => {
          const step = `${(await state.viewStart + i) % await state.resolution + 1}/${await state.resolution}`;
          switch (await state[`view:${i}`]) {
            case 0: return '';
            case 1: return `${step} _`;
            case 2: return `${step} █`;
            case 3: return `${step} ░`;
          }
        },
        onDown: () => send('stepSequence', i),
      })),
      ['K', title(async () => `bar ${((await state.viewStart / await state.resolution)|0) + 1}/${await state.bars}`) ],
    ])}],
    ['S', { mode: 'EQ', actions: new Map([
      ...oneOf('YUIOP', state, 'eqBand', ['hi pass', 'mid 1', 'mid 2', 'mid 3', 'lo pass']),
      ...oneOf('NM', state, 'eqFilter', ['freq.', 'res.']),
      ...nudge(async () => await state[`${await state.eqBand}:${await state.eqFilter}`], i => {}),
      fill,
    ])}],
    ['D', { mode: 'ADSR', actions: new Map([
      ...oneOf('YUIO', state, 'adsr', ['attack', 'decay', 'sustain', 'release']),
      ...nudge(async () => await state[await state.adsr], i => {}),
      fill,
    ])}],
    ['F', { mode: 'FX', actions: new Map([
      ...oneOf('YUIOP', state, 'fx', ['comp.', 'distort', 'vocoder', 'chorus', 'duck']),
      ...nudge(async () => await state[await state.fx], i => {}),
      fill,
    ])}],
    ['G', { mode: 'Mix', actions: new Map([
      ...oneOf('YUIOPNM,', state, 'mix', ['volume', 'send 1', 'send 2', 'send 3', 'send 4', 'pan', 'to duck', 'to tape']),
      ...nudge(async () => await state[await state.mix], i => send(state.mix, i)),
      fill,
    ])}],
    ['Z', { mode: 'Song', actions: new Map([
      ...oneOf('Y', state, 'song', ['tempo']),
      ...nudge(async () => await state[await state.song], async i => send(await state.song, i)),
      ['N', toggle('play', async () => await state.playing, () => send('play')) ],
      ['M', toggle('arm', async () => await state.armed, () => send('arm')) ],
    ])}],
    ['X', { mode: 'Auto', actions: new Map([
    ])}],
    ['C', { mode: 'Send', actions: new Map([
    ])}],
    ['V', { mode: 'Mute', actions: new Map([
    ])}],
    ['B', { mode: 'Tape', actions: new Map([
    ])}],
    [undefined, { actions: new Map([
      ...all(i => ({
        onDown: () => send('auditionDown', i),
        onUp: () => send('auditionUp', i),
      })),
    ])}],
  ]);
};
