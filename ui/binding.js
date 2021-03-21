class Binding {
  static _none() {
    return '';
  }

  static oneOf(caps, state, name, labels) {
    state[name] = labels[0];
    return Binding.group(caps, i => ({
      label: () => labels[i],
      title: () => state[name] === labels[i],
      onDown: () => state[name] = labels[i],
    }));
  }

  static nudge(caps, jump, onDown) {
    return Binding.group(caps, i => ({ label: () => [-jump, -1, 1, '+' + jump][i], onDown: () => onDown(i) }));
  }

  static group(caps, f) {
    return Array.from(caps, (cap, i) => [cap, new Binding(f(i))])
  }

  static title(label) {
    return new Binding({ label, title: () => true });
  }

  static toggle(label, title, onDown) {
    return new Binding({ label: () => label, title, onDown });
  }

  constructor(options = {}) {
    this.label = options.label ?? Binding._none;
    this.title = options.title ?? Binding._none;
    this.onDown = options.onDown ?? Binding._none;
    this.onUp = options.onUp ?? Binding._none;
  }
}
