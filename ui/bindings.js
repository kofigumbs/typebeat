const Bindings = ({ state, send }) => {
  const method = (...parts) => parts.join(':').replace(/[^\w:]/, '');

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

  return new Map([
    ['Q', { actions: new Map([
      ...all(i => ({
        label: async () => i === await state.activeTrack ? 'active' : '',
        title: async () => i === await state.activeTrack,
        onDown: () => send('activeTrack', i),
      })),
    ])}],
    ['W', { mode: 'Sound', actions: new Map([
      ...oneOf('YUIO', state, 'sound', ['sample', 'synth 1', 'synth 2', 'synth 3']),
      ...oneOf('NM,', state, 'soundControl', ['type', 'level', 'detune']),
      ...group('HJKL;', i => {
        const soundMethod = () => method(state.sound, state.soundControl);
        const soundNudge = nudge(() => state[soundMethod()], j => send(soundMethod(), j))[i][1];
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
            state.soundControl === 'type' ? send(soundMethod(), i) : soundNudge.onDown();
          },
        };
      }),
    ])}],
    ['E', { mode: 'Chop', actions: new Map([
    ])}],
    ['R', { mode: 'Poly', actions: new Map([
    ])}],
    ['T', { mode: 'Note', actions: new Map([
      ...all(i => ({
        label: async () => {
          const note = await state[method('note', i)];
          const name = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][note % 12];
          const octave = Math.floor(note / 12 - 1);
          return `${name}${octave}`;
        },
        title: async () => await state[method('note', i)] === await state.naturalNote,
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
      ...oneOf('YUIOP', state, 'eqBand', ['low', 'band 1', 'band 2', 'band 3', 'high']),
      ...oneOf('NM', state, 'eqFilter', ['freq.', 'res.']),
      ...nudge(() => state[method(state.eqBand, state.eqFilter)], i => send(method(state.eqBand, state.eqFilter), i)),
    ])}],
    ['D', { mode: 'Hold', actions: new Map([
      ...oneOf('YUIO', state, 'hold', ['attack', 'decay', 'sustain', 'release']),
      ...nudge(async () => await state[state.hold], i => send(state.hold, i)),
      ['N', toggle('sample', async () => await state.holdSample, () => send('holdSample')) ],
    ])}],
    ['F', { mode: 'FX', actions: new Map([
    ])}],
    ['G', { mode: 'Mix', actions: new Map([
      ...oneOf('YU,', state, 'mix', ['volume', 'pan']),
      ...nudge(async () => await state[state.mix], i => send(state.mix, i)),
    ])}],
    ['Z', { mode: 'Song', actions: new Map([
      ...oneOf('Y', state, 'song', ['tempo']),
      ...nudge(async () => await state[state.song], async i => send(state.song, i)),
      ['P', bind({
        label: () => 'tap',
        title: () => !!state.tempoTaps.length,
        onDown: (time) => {
          state.tempoTaps.push(time);
          if (state.tempoTaps.length === 1)
            return;
          let diffs = 0;
          for (let i = 1; i < state.tempoTaps.length; i++)
            diffs += state.tempoTaps[i] - state.tempoTaps[i - 1];
          send('tempoTaps', Math.round(60000 / (diffs / (state.tempoTaps.length - 1)) + 1));
        },
      })],
      ['N', toggle('play', async () => await state.playing, () => send('play')) ],
      ['M', toggle('arm', async () => await state.armed, () => send('arm')) ],
    ])}],
    ['X', { mode: 'LFO', actions: new Map([
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
