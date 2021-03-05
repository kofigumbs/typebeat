class Binding {
  static titleGroup(caps, titles, state, field) {
    return Binding.group(caps, i => ({
      label: () => titles[i],
      title: () => (state[field] ?? 0) === i,
      onDown: () => state[field] = i,
    }));
  }

  static nudgeGroup(send = Binding._noop, getIndex, state, field) {
    return [
      ['H', new Binding({ label: () => '-10', onDown: () => send(`nudge:${field}`, (getIndex() << 4) | 0) })],
      ['J', new Binding({ label: () => '-1',  onDown: () => send(`nudge:${field}`, (getIndex() << 4) | 1) })],
      ['K', new Binding({ label: () => state[field][getIndex()], title: () => true })],
      ['L', new Binding({ label: () => '+1',  onDown: () => send(`nudge:${field}`, (getIndex() << 4) | 2) })],
      [';', new Binding({ label: () => '+10', onDown: () => send(`nudge:${field}`, (getIndex() << 4) | 3) })],
    ];
  }

  static group(caps, f) {
    return Array.from(caps, (cap, i) => [cap, new Binding(f(i))])
  }

  static fill() {
    return [
      ['/', new Binding({ label: () => 'FILL' }) ],
    ];
  }

  static _noop() {
    return '';
  }

  constructor(options = {}) {
    this.label = options.label ?? Binding._noop;
    this.title = options.title ?? Binding._noop;
    this.onDown = options.onDown ?? Binding._noop;
    this.onUp = options.onUp ?? Binding._noop;
  }
}
