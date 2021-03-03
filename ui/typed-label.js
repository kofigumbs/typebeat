customElements.define('typed-label', class extends HTMLElement {
  static get observedAttributes() {
    return ['aria-label'];
  }

  connectedCallback() {
    this._timeoutIds = [];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    oldValue = oldValue || '';
    if (oldValue === newValue)
      return;
    this._timeoutIds.forEach(clearTimeout);
    this._timeoutIds = [];
    const innerText = this.innerText;
    let shared = 0;
    while (shared < innerText.length && shared < newValue.length && innerText[shared] === newValue[shared])
      shared++;
    for (const char of innerText.substring(shared))
      this._type(s => s.slice(0, -1));
    for (const char of newValue.substring(shared))
      this._type(s => s + char);
  }

  _type(callback) {
    this._timeoutIds.push(
      setTimeout(
        () => this.innerText = callback(this.innerText),
        40*this._timeoutIds.length + 20*Math.random()
      )
    );
  }
});
