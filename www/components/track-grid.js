export default class TrackGrid extends HTMLElement {
  connectedCallback() {
    const width = 2;
    const length = width * 6;
    const offset = length + 2;
    const row = ({ x, y, id }) => `
      <path id="track-${id + 0}" d="M ${x + 0 * offset} ${y} h ${length}" stroke-width="${width}"></path>
      <path id="track-${id + 1}" d="M ${x + 1 * offset} ${y} h ${length}" stroke-width="${width}"></path>
      <path id="track-${id + 2}" d="M ${x + 2 * offset} ${y} h ${length}" stroke-width="${width}"></path>
      <path id="track-${id + 3}" d="M ${x + 3 * offset} ${y} h ${length}" stroke-width="${width}"></path>
      <path id="track-${id + 4}" d="M ${x + 4 * offset} ${y} h ${length}" stroke-width="${width}"></path>
    `;
    const grid = ({ x, y }) =>
      row({ x, y, id: 10 }) +
        row({ x: x + length/3, y: y + length, id: 5 }) +
        row({ x: x + length/3*2, y: y + length*2, id: 0 });
    this.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg" style="width: 100%; height: 100%;">
        <style>
          path {
            transform: translate(0, 0);
            stroke: var(--key_background);
            stroke-width: ${width}px;
            --key_background: var(--dark);
            --key_pulse: transparent;
          }
          path.active {
            stroke-width: ${length}px;
          }
        </style>
        ${grid({ x: 9, y: 11 })}
      </svg>
    `;
    this._tracks = Array.from({ length: 15 }).map((_, i) => this.querySelector(`#track-${i}`));
  }
}
