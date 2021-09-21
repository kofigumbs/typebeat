import init from 'typebeat-ui/index.js';
import wasm from '../target/wasm32-unknown-emscripten/release/typebeat_web.wasm?url';
import sections from './GUIDE.md'
import './index.css';

// This is probably a bug with Emscripten. Seems like it's missing a `var`
// before using miniaudio. I don't understand it well enough to report tho.
globalThis.miniaudio = undefined;

let step = 0;
const guide = document.querySelector('.guide');
const setContent = () => guide.innerHTML = sections[step].content;
setContent();

const advance = (x) => {
  if (sections[step].checks.every(([field, value]) => new RegExp(value).test(x[field]))) {
    step++;
    setContent();
  }
};

const mount = document.querySelector('.mount');
const rows = Array.from(mount.children);
const resize = () => requestAnimationFrame(() => {
  const left = Math.min(...rows.map(el => el.offsetLeft));
  const right = Math.max(...rows.map(el => el.offsetLeft + el.offsetWidth));
  const scale = Math.min(1, mount.parentElement.offsetWidth / (right - left));
  const margin = `${-.5 * mount.offsetHeight * (1 - scale)}px`;
  mount.style.transform = `scale(${scale})`;
  mount.style.marginTop = mount.style.marginBottom = margin;
});
window.addEventListener("resize", resize);
window.addEventListener("DOMContentLoaded", resize);


import('../target/wasm32-unknown-emscripten/release/typebeat-web.js').then(async factory => {
  const lib = await factory.default({ locateFile: () => wasm, noExitRuntime: true });
  const start = () => {
    const controller = lib.ccall('start', 'number', [], []);
    advance();
    init((context, { method, data }) => {
      advance({ context, method, data });
      switch (context) {
        case 'get': return lib.ccall('get', 'number', ['number', 'string'], [controller, method]);
        case 'set': return lib.ccall('set', 'number', ['number', 'string', 'number'], [controller, method, data]);
      }
    });
  };
  const handleKey = event => {
    if (event.key === ' ') {
      start();
      document.removeEventListener('keypress', handleKey);
    }
  };
  document.addEventListener('keypress', handleKey);
  document.querySelector('.guide kbd').addEventListener('click', start);
});
