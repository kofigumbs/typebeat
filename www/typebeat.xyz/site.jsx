import { createEffect, createMemo, createSignal, onMount } from 'solid-js';

import wasm from '../../target/wasm32-unknown-emscripten/release/typebeat_dot_xyz.wasm?url';

import App, { createEventListener, modes } from '../app.jsx'
import './index.css';

// FIXME(https://github.com/mackron/miniaudio/issues/363)
globalThis.miniaudio = undefined;

const [started, setStarted] = createSignal(false);

const lib = import('../../target/wasm32-unknown-emscripten/release/typebeat-dot-xyz.js')
  .then(factory => factory.default({ locateFile: () => wasm, noExitRuntime: true }))
  .then(({ ccall, _free, UTF8ToString }) => {
    // Parse the char* return value then free it
    const getJson = (method) => {
      const ptr = ccall(method, 'number');
      const obj = JSON.parse(UTF8ToString(ptr));
      _free(ptr);
      return obj;
    };

    // Only start the typebeat device once we receive an command
    const send = (method, data) => {
      if (!started()) {
        ccall('typebeat_start', null)
        setStarted(true);
      }
      return ccall('typebeat_send', null, ['string', 'number'], [method, data]);
    };

    // If Typebeat is running, the browser might prevent you from navigating away
    const stop = () => {
      if (started())
        ccall('typebeat_stop', null);
    };
    if (import.meta.hot)
      import.meta.hot.dispose(stop);
    window.addEventListener('beforeunload', stop);

    const onChange = callback => {
      callback([getJson('typebeat_dump')]);
      (function poll() {
        getJson('typebeat_changes').forEach(callback);
        requestIdleCallback(poll)
      })();
    };

    return { send, onChange };
  });

const Block = (props) => (
  <Dynamic
    component={props.tagName || 'p'}
    className={`copy full-width spaced-above spaced-below ${props.className || ''}`}
  >
    {props.children}
  </Dynamic>
);

const Intro = (props) => (
  <Block>
    Want to try it first?
    This page is interactive!
    Try pressing <kbd>K</kbd> to trigger a sample (works best on desktop Chrome).
  </Block>
);

export default () => {
  const [modifier, setModifier] = createSignal();
  const showHelp = createMemo(previous => !!previous + !!modifier() + started());

  let container;
  createEffect(() => {
    if (showHelp())
      container.scrollIntoView({ behavior: 'smooth' });
  });

  let app;
  onMount(() => {
    const rows = Array.from(app.children);
    const resize = () => {
      const left = Math.min(...rows.map(el => el.offsetLeft));
      const right = Math.max(...rows.map(el => el.offsetLeft + el.offsetWidth));
      const scale = Math.min(1, app.parentElement.offsetWidth / (right - left));
      const margin = `${-.5 * app.offsetHeight * (1 - scale)}px`;
      app.style.transform = `scale(${scale})`;
      app.style.marginTop = app.style.marginBottom = margin;
    };
    resize();
    createEventListener(window, 'resize', resize);
  });

  return (
    <div ref={container} class='column full-height'>
      <br className='spaced-below' />
      <div ref={app} className='app'>
        <App
          init={(state) => createEffect(() => setModifier(state.modifier))}
          send={(method, data) => lib.then(lib => lib.send(method, data))}
          onChange={(callback) => lib.then(lib => lib.onChange(callback))}
        />
      </div>
      <div className='expanded padded-horizontally spaced-above spaced-below'>
        <Show when={showHelp()} fallback={<Intro />}>
          {modes.get(modifier()).Help({ Block })}
        </Show>
      </div>
      <footer className='padded-horizontally'>
        <Block>
          (C) {new Date().getFullYear()} <a href="https://kofi.sexy">Kofi Gumbs</a>
        </Block>
      </footer>
    </div>
  );
};
