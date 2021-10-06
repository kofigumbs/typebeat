const width = 2;
const length = width * 7;
const offset = length + 5;
const row = (n) => Array.from({ length: 5 }).map((_, i) => ({
  x: n*length/3 + i*offset - 1,
  y: (2*n+1)*length/2 + n + n*width - 1,
}));

const grid = row(2).concat(row(1), row(0));
export const inert = grid.map(({ x, y }) => `M ${x} ${y} h ${length}`);
export const active = grid.map(({ x, y }) => `M ${x} ${y - length/2} h ${length} v ${length} h -${length} Z`);
export const muted = grid.map(({ x, y }) => `M ${x} ${y} l ${length/2} -${length/2} l ${length/2} ${length/2} l -${length/2} ${length/2} Z`);
