import init from '../../src/client';
import wasm from '../../target/wasm32-unknown-emscripten/release/typebeat_web.wasm?url';
import sections from './GUIDE.md'
import './index.css';

// FIXME(https://github.com/mackron/miniaudio/issues/363)
globalThis.miniaudio = undefined;


// Setup guide
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


// Resize mount node so that it never requires horizontal scroll
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
window.addEventListener('resize', resize);
window.addEventListener('DOMContentLoaded', resize);


// Start loading the (large) JS runtime
const lib = import('../../target/wasm32-unknown-emscripten/release/typebeat-web.js').then(factory => {
  return factory.default({ locateFile: () => wasm, noExitRuntime: true });
});


// If Typebeat is running, sometimes the browser won't let you navigate away immediately.
// This doesn't seem to happen if we explicitly stop the audio device.
lib.then(() => {
  window.addEventListener('beforeunload', () => controller.ccall('stop', null))
});


// Setup the main app, but only start the audio device once we receive a set.
// Until then, all gets return 0.
let started = false;
lib.then(controller => init((context, { method, data }) => {
  advance({ context, method, data });
  switch (context) {
    case 'get':
      if (!started) {
        return 0;
      }
      return controller.ccall('get', 'number', ['string'], [method]);
    case 'set':
      if (!started) {
        controller.ccall('start', null);
        started = true;
      }
      return controller.ccall('set', null, ['string', 'number'], [method, data]);
  }
}));
