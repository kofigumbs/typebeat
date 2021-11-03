export const pulse = el => {
  el.classList.remove('pulse');
  void el.getBoundingClientRect(); // trigger a DOM reflow
  el.classList.add('pulse');
  el.addEventListener('animationend', event => event.target.classList.remove('pulse'), { once: true });
};

const typeDelay = (setValue, cache, callback) => {
  cache.timeoutIds.push(
    setTimeout(
      () => setValue(cache.value = callback(cache.value)),
      60*cache.timeoutIds.length + 20*Math.random())
  );
}

export const type = (newValue = '', oldValue = '', setValue, cache) => {
  if (oldValue === newValue)
    return;
  let id;
  while (id = cache.timeoutIds.pop())
    clearTimeout(id);
  let shared = 0;
  while (shared < cache.value.length && shared < newValue.length && cache.value[shared] === newValue[shared])
    shared++;
  for (const char of cache.value.substring(shared))
    typeDelay(setValue, cache, s => s.slice(0, -1));
  for (const char of newValue.substring(shared))
    typeDelay(setValue, cache, s => s + char);
};
