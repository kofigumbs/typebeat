import Actions from '../actions';

export const cap = 'V';

export const actions = Actions.comingSoon;

customElements.define('tape-mode', class extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <circle cx="73" cy="23" r="24" fill="none" stroke-width="2"></circle>
        <circle cx="23" cy="23" r="24" fill="none" stroke-width="2"></circle>
      </svg>
    `;
  }
});
