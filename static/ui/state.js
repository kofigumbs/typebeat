window.State = ({ get, defaults }) => {
  const local = new Map(defaults);
  const cache = new Map();
  return [
    new Proxy({}, {
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
    }),
    () => cache.clear(),
  ];
};
