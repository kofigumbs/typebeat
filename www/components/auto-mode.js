import bind from '../bind';

export const cap = 'X';

export const actions = (local, proxy, set) => bind.comingSoon;

customElements.define('auto-mode', class extends HTMLElement {
  connectedCallback() {
    const ls = Array.from({ length: 50 }).map((_, i, a) => `l 1 ${Math.cos(i/a.length*2*Math.PI)}`);
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <path d="M 23 23 ${ls.join('')}" stroke-width="2" fill="none"></path>
      </svg>
    `;
  }
});
