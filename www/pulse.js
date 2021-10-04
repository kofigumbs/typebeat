export default el => {
  el.classList.remove('pulse');
  void el.getBoundingClientRect(); // trigger a DOM reflow
  el.classList.add('pulse');
  el.addEventListener('animationend', () => el.classList.remove('pulse'), { once: true })
};
