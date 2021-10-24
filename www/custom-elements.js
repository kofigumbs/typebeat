import Tare from 'tare';

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
    while (shared < this._innerText.length && shared < newValue.length && this._innerText[shared] === newValue[shared])
      shared++;
    for (const char of this._innerText.substring(shared))
      this._type(s => s.slice(0, -1));
    for (const char of newValue.substring(shared))
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
