import './FiraCode/distr/fira_code.css';
import './index.css';
import './Themes/scripts/lib/theme.js';
import './tare/dist/tare.min.js';
import './typed-label.js';


/*
 * state (global)
 */

const State = ({ get }) => {
  const cache = new Map();
  const local = new Map([
    ['invalidate', cache.clear.bind(cache)],
    ['modifier', undefined],
    ['tempoTaps', []],
  ]);
  return new Proxy({}, {
    get: function(target, prop) {
      if (local.has(prop))
        return local.get(prop);
      else if (cache.has(prop))
        return cache.get(prop)
      const value = get(prop);
      cache.set(prop, value);
      return value;
    },
    set: function(target, prop, value) {
      local.set(prop, value);
      return true;
    },
  });
};


/*
 * bindings (global)
 */

const Bindings = ({ set }) => {
  // join two method name parts with camel case
  const join = (a, b) => `${a}${b[0].toUpperCase()}${b.substring(1)}`.replace(/[^\w]/, '');

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
        const soundMethod = () => join(state.sound, state.soundControl);
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
        label: async () => note(await state[`note ${i}`]),
        title: async () => i == await state.activeKey,
        onDown: () => set('noteDown', i),
        onUp: () => set('noteUp', i),
      })),
    ])}],
    ['A', { mode: 'Beat', actions: new Map([
      ['Y', title(() => 'tempo')],
      ...nudge(() => state.tempo, i => set('tempo', i)),
      ['N', toggle('play', () => state.playing, () => set('playing')) ],
      ['M', toggle('arm', () => state.armed, () => set('armed')) ],
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
          switch (await state[`view ${i}`]) {
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
      ...nudge(() => state[join(state.eqBand, state.eqFilter)], i => set(join(state.eqBand, state.eqFilter), i)),
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
      ...nudge(() => state[join(state.effect, state.effectControl)], i => set(join(state.effect, state.effectControl), i)),
      ...oneOf('NM,', state, 'effectControl', ['gain', 'feed', 'space']),
    ])}],
    ['V', { mode: 'Tape', actions: new Map([
    ])}],
    ['B', { mode: 'Mute', actions: new Map([
      ...all(i => ({
        label: async () => await state[`muted ${i}`] ? '</>' : '==',
        onDown: () => set('muted', i),
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


/*
 * sync
 */

const forceClass = (el, className) => {
  el.classList.remove(className);
  void el.offsetWidth; // trigger a DOM reflow
  el.classList.add(className);
}

const sync = async () => {
  state.invalidate();
  const binding = bindings.get(state.modifier);
  for (let i = 0; i < keysOnRight.length; i++) {
    const key = keysOnRight[i];
    const action = binding.actions.get(key.dataset.cap);
    labels[i].ariaLabel = await action?.label() ?? '';
    if (!!state.modifier)
      labels[i].classList.toggle('title', !!(await action?.title()));
    if (await state[`recent ${i}`])
      forceClass(minipads[i], 'pulse');
    minipads[i].classList.toggle('active', i === await state.activeTrack);
  };
  if (await state.playing)
    requestSync();
};

let nextSyncId;
const clearNextSyncIdAndSync = () => {
  nextSyncId = null;
  sync();
};
const requestSync = () => {
  if (!nextSyncId)
    nextSyncId = requestAnimationFrame(clearNextSyncIdAndSync);
};


/*
 * events
 */

const hasModifier = event => (
  event.ctrlKey || event.metaKey || event.shiftKey || event.altKey
);

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const handleDocumentKey = event => {
  if (hasModifier(event))
    return;
  event.preventDefault();
  if (event.repeat)
    return;
  const cap = capsByEventCode.get(event.code);
  if (!cap)
    return;
  const down = event.type === 'keydown';
  const binding = bindings.has(cap);
  if (binding && !down)
    return;
  if (binding) {
    state.modifier = state.modifier === cap ? undefined : cap;
    for (const key of keysOnLeft)
      key.classList.toggle('mode', !!state.modifier && key.dataset.cap === state.modifier);
    state.tempoTaps = [];
  }
  else {
    const handler = bindings.get(state.modifier).actions.get(cap);
    down ? handler?.onDown(event.timeStamp) : handler?.onUp(event.timeStamp);
    if (down)
      forceClass(keysOnRight.find(key => cap === key.dataset.cap), 'pulse');
  }
  requestSync();
};

export default (callback) => {
  const theme = new window.Theme();
  theme.install();
  theme.start();
  theme.onLoad = () => {
    for (const [name, value] of Object.entries(theme.active))
      document.documentElement.style.setProperty(`--${name}`, value);
  };

  globalThis.state = State({ get: method => callback('get', { method }) });
  globalThis.bindings = Bindings({ set: (method, data) => callback('set', { method, data }) });

  const capsOnLeft = 'ZXCVBASDFGQWERT';
  const capsOnRight = 'NM,./HJKL;YUIOP';
  const mapJoin = (iterable, f) => Array.from(iterable).map(f).join('');
  document.querySelector('.mount').innerHTML += mapJoin(['QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./'], row => `
    <div class="row">
      ${mapJoin(row, cap => {
        if (cap === 'Q')
          return `
            <div class="key" data-cap="${cap}">
              ${mapJoin(capsOnRight.match(/.{1,5}/g).reverse(), minirow => `
                <div class="minirow">
                  ${mapJoin(minirow, c => `<div class="minipad" data-cap="${c}"></div>`)}
                </div>`
              )}
            </div>
          `;
        else if (bindings.has(cap))
          return `
            <div class="key" data-cap="${cap}">
              ${Tare.html(bindings.get(cap).mode)}
            </div>
          `;
        else
          return `
            <div class="key pad" data-cap="${cap}">
              <typed-label class="label" aria-label=""></typed-label>
            </div>
          `;
      })}
    </div>
  `);

  const findElements = (caps, f) => Array.from(caps).map(cap => document.querySelector(f(cap)));
  globalThis.keysOnLeft = findElements(capsOnLeft, cap => `.key[data-cap="${cap}"]`);
  globalThis.keysOnRight = findElements(capsOnRight, cap => `.key[data-cap="${cap}"]`);
  globalThis.labels = findElements(capsOnRight, cap => `.key[data-cap="${cap}"] .label`);
  globalThis.minipads = findElements(capsOnRight, cap => `.minipad[data-cap="${cap}"]`);

  document.addEventListener('keydown', handleDocumentKey);
  document.addEventListener('keyup', handleDocumentKey);
  document.addEventListener('keypress', event => !hasModifier(event));
  sync();
};
