import 'firacode';
import Tare from 'tare';
import './index.css';

customElements.define('custom-element-tare', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = Tare.html(this.getAttribute('aria-label'));
  }
});

customElements.define('custom-element-mono', class extends HTMLElement {
  static get observedAttributes() {
    return ['aria-label'];
  }

  connectedCallback() {
    this._innerText = ''; // cache to avoid extra DOM reflows
    this._timeoutIds = [];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if ((oldValue ?? '') === newValue)
      return;
    this._timeoutIds.forEach(clearTimeout);
    this._timeoutIds = [];
    let shared = 0;
    const max = Math.min(this._innerText.length, newValue.length);
    while (shared < max && this._innerText[shared] === newValue[shared])
      shared++;
    for (let char of this._innerText.substring(shared))
      this._type(s => s.slice(0, -1));
    for (let char of newValue.substring(shared))
      this._type(s => s + char);
  }

  _type(callback) {
    this._timeoutIds.push(
      setTimeout(
        () => this.innerText = this._innerText = callback(this._innerText),
        40*this._timeoutIds.length + 20*Math.random())
    );
  }
});

// https://github.com/elm/browser/issues/77
// https://github.com/elm/browser/issues/89
const capsByEventCode = new Set([
  'Semicolon', 'Comma', 'Period', 'Slash',
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => `Key${cap}`),
]);
const isAppKey = (event) => {
  return capsByEventCode.has(event.code) &&
    !(event.ctrlKey || event.metaKey || event.shiftKey || event.altKey);
}
const isAppEvent = (event) => {
  if (isAppKey(event)) {
    event.preventDefault();
    if (!event.repeat)
      return true;
  }
};
export const onKeyboardEvent = (callback) => {
  document.addEventListener('keydown', event => isAppEvent(event) && callback(event));
  document.addEventListener('keyup', event => isAppEvent(event) && callback(event));
  document.addEventListener('keypress', event => !isAppKey(event));
};
