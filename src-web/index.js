import './index.css';
import init from 'typebeat-ui/index.js';
import wasm from '../target/wasm32-unknown-emscripten/release/typebeat_web.wasm?url';

// This is probably a bug with emscripten. Seems like it's missing a `var`
// before using miniaudio. I don't understand it well enough to report tho.
globalThis.miniaudio = undefined;

import('../target/wasm32-unknown-emscripten/release/typebeat-web.js').then(async factory => {
  const lib = await factory.default({ locateFile: () => wasm, noExitRuntime: true });
  const start = () => {
    document.querySelector('.info').remove();
    document.body.classList.add('mount');
    const controller = lib.ccall('start', 'number', [], []);
    init((context, { method, data }) => {
      switch (context) {
        case 'get': return lib.ccall('get', 'number', ['number', 'string'], [controller, method]);
        case 'set': return lib.ccall('set', 'number', ['number', 'string', 'number'], [controller, method, data]);
      }
    });
  };
  document.addEventListener('keypress', start, { once: true });
});
