export default ({ scope }) => {
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
  return `
    <svg xmlns="http://www.w3.org/2000/svg">
      <style>
        ${scope} path {
          stroke: var(--key_background);
          --key_background: var(--dark);
          --key_pulse: transparent;
          transform: translate(9px, 11px);
        }
        ${scope} path.active {
          stroke-width: ${length}px;
        }
      </style>
      ${
        row({ x: 0, y: 0, id: 10 }) +
          row({ x: length/3, y: length, id: 5 }) +
          row({ x: length/3*2, y: length*2, id: 0 })
      }
    </svg>
  `;
};
