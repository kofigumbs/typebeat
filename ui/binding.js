class Binding {
  static _none() {
    return '';
  }

  static tabs(caps, state, name, labels) {
    state.tab = { ...(state.tab ?? {}), [name]: 0 };
    return Binding.group(caps, i => ({
      label: () => labels[i],
      title: () => state.tab[name] === i,
      onDown: () => state.tab[name] = i,
    }));
  }

  static buttons(caps, labels, onDown) {
    return Binding.group(caps, i => ({ label: () => labels()[i], onDown: () => onDown(i) }));
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
