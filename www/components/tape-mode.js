import bind from '../bind';

export const cap = 'V';

export const actions = (local, proxy, set) => bind.comingSoon;

customElements.define('tape-mode', class extends HTMLElement {
  connectedCallback() {
    const y = 23;
    const circles = [{ x: 23,  r: 16 }, { x: 23,  r: 8 }, { x: 73, r: 24 }, { x: 73, r: 8 }];
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg">
        <style>
          tape-mode.visual circle:nth-of-type(odd) {
            fill: var(--dark);
          }
          tape-mode.visual circle:nth-of-type(even) {
            fill: var(--key_background);
          }
        </style>
        ${circles.map(({ x, r, cls }) => `<circle cx="${x}" cy="${y}" r="${r}"></circle>`).join('')}
      </svg>
    `;
  }
});
