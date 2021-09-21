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

let step = -1;
const tutorial = document.querySelector('.tutorial');
const advance = () => {
  step++;
  tutorial.innerHTML = content[step][0];
};
advance();

const attemptAdvance = (method, data) => {
  const current = content[step];
  if (current && current[1] === method && current[2] === data)
    advance();
};

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
          attemptAdvance(method, data);
          return lib.ccall('set', 'number', ['number', 'string', 'number'], [controller, method, data]);
      }
    });
  };
  document.addEventListener('keypress', start, { once: true });
});
