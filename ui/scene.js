const scene = new Zdog.Anchor();
const placeholder = anchor => new Zdog.Rect({ addTo: anchor, color: lightOrange, width: 50, height: 50, stroke: 10 });

const at = (v, callback) => {
  const anchor = new Zdog.Anchor({
    addTo: scene,
    rotate: { x: v.y*-.004, y: v.x*-.004 },
    translate: { x: v.x, y: v.y, z: -.2*(new Zdog.Vector(v).magnitude()) },
  });
  callback(anchor);
  return anchor;
};

const models = new Map([
  ['Q', at({ x: -180, y: -75 }, placeholder)],
  ['W', at({ x: -100, y: -80 }, placeholder)],
  ['E', at({ x:  -20, y: -85 }, placeholder)],
  ['R', at({ x:   60, y: -80 }, placeholder)],
  ['T', at({ x:  140, y: -75 }, placeholder)],
  ['A', at({ x: -160, y:   0 }, placeholder)],
  ['s', at({ x:  -80, y:   0 }, placeholder)],
  ['D', at({ x:    0, y:   0 }, placeholder)],
  ['F', at({ x:   80, y:   0 }, placeholder)],
  ['G', at({ x:  160, y:   0 }, placeholder)],
  ['Z', at({ x: -140, y:  75 }, placeholder)],
  ['X', at({ x:  -60, y:  80 }, placeholder)],
  ['C', at({ x:   20, y:  85 }, placeholder)],
  ['V', at({ x:  100, y:  80 }, placeholder)],
  ['B', at({ x:  180, y:  75 }, placeholder)],
]);
