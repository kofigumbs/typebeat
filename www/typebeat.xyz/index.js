import init from '..';
import wasm from '../../target/wasm32-unknown-emscripten/release/typebeat_dot_xyz.wasm?url';
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
document.addEventListener('keypress', event => advance({ code: event.code }));
guide.querySelector('button').addEventListener('click', () => advance({ code: 'Space' }));


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


// Setup keyboard label toggle
const toggle = document.querySelector('.toggle-labels');
toggle.addEventListener('click', () => {
  toggle.innerText = toggle.innerText.replace(
    /Show|Hide/, 
    mount.classList.toggle('labeled') ? 'Hide' : 'Show'
  );
});


// Start loading the (large) JS runtime
import('../../target/wasm32-unknown-emscripten/release/typebeat-dot-xyz.js')
  .then(factory => factory.default({ locateFile: () => wasm, noExitRuntime: true }))
  .then(({ ccall, _free, UTF8ToString }) => {
    // If Typebeat is running, the browser might prevent you from navigating away
    window.addEventListener('beforeunload', () => ccall('typebeat_stop', null));

    // Parse the char* return value then free it
    const getJson = (method) => {
      const ptr = ccall(method, 'number');
      const obj = JSON.parse(UTF8ToString(ptr));
      _free(ptr);
      return obj;
    }

    // Setup the main app, but only start the typebeat device once we receive a set
    let started = false;
    const update = init(getJson('typebeat_dump'), (method, data) => {
      advance({ method, data });
      if (!started) {
        ccall('typebeat_start', null)
        started = true;
      }
      return ccall('typebeat_send', null, ['string', 'number'], [method, data]);
    });

    // Start polling for state changes
    (function poll() {
      getJson('typebeat_changes').forEach(update);
      requestIdleCallback(poll)
    })();
  });
