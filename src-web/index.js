import init from 'typebeat-ui/index.js';
import wasm from '../target/wasm32-unknown-emscripten/release/typebeat_web.wasm?url';
import './index.css';

// This is probably a bug with Emscripten. Seems like it's missing a `var`
// before using miniaudio. I don't understand it well enough to report tho.
globalThis.miniaudio = undefined;

const content = [
  [`
    <p>
      Press <kbd>SPACE</kbd> to start the tutorial.
    </p>
  `],
  [`
    <p>
      OK, let's start by triggering a sound. Typebeat's controls are laid out in
      two halves. By default, any key on the right half of your keyboard will
      trigger sound.
    </p>
    <p>
      Try tapping the <kbd>K</kbd> key on your keyboard to trigger a clap.
    </p>
  `, 'auditionDown', 7],
  [`
    <p>
      Nice!
    </p>
  `],
];

let step = 0;
const tutorial = document.querySelector('.tutorial');
const setContent = () => tutorial.innerHTML = content[step][0];
setContent();

const advance = (method, data) => {
  const current = content[step];
  if (current && current[1] === method && current[2] === data) {
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
  const start = event => {
    if (event.key !== ' ')
      return;
    const controller = lib.ccall('start', 'number', [], []);
    advance();
    init((context, { method, data }) => {
      switch (context) {
        case 'get':
          return lib.ccall('get', 'number', ['number', 'string'], [controller, method]);
        case 'set':
          advance(method, data);
          return lib.ccall('set', 'number', ['number', 'string', 'number'], [controller, method, data]);
      }
    });
  };
  document.addEventListener('keypress', start, { once: true });
});
