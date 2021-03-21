function State({ receive, defaults }) {
  const local = new Map(defaults);
  return new Proxy({}, {
    get: function(target, prop) {
      return local.has(prop) ? local.get(prop) : receive(prop);
    },
    set: function(target, prop, value) {
      local.set(prop, value);
      return true;
    },
  });
}
