import { createEffect, createMemo, createSignal, onMount } from 'solid-js';

import wasm from '../../target/wasm32-unknown-emscripten/release/typebeat_dot_xyz.wasm?url';

import App, { createEventListener } from '../app.jsx'
import guide from './guide.jsx';
import './index.css';

// FIXME(https://github.com/mackron/miniaudio/issues/363)
globalThis.miniaudio = undefined;

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

    // Only start the typebeat device once we receive an action
    let started = false;
    const send = (method, data) => {
      if (!started) {
        ccall('typebeat_start', null)
        started = true;
      }
      return ccall('typebeat_send', null, ['string', 'number'], [method, data]);
    };

    // If Typebeat is running, the browser might prevent you from navigating away
    const stop = () => {
      if (started)
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

const Arrow = props => {
  const next = createMemo(() => props.page + props.step);
  const disabled = createMemo(() => next() < 0 || next() >= guide.length);
  return (
    <button
      className='bold reset'
      classList={{ faded: disabled() }}
      disabled={disabled()}
      onClick={() => props.setPage(next())}
    >
      {props.children}
    </button>
  );
};

const Guide = props => {
  const [page, setPage] = createSignal(0);
  const advance = event => setPage(page => {
    const { until: key, is: value } = guide[page];
    return page + (
      Array.isArray(value) ? value.includes(event[key]) : value === event[key]
    );
  });

  createEffect(() => advance(props.appEvent));
  createEventListener(document, 'keypress', event => advance({ keypress: event.code }));

  const contents = Array.from(guide).map(content => content.render({ advance }));
  for (let el of contents.flat()) {
    el.classList.add('copy', 'full-width');
    for (let kbd of el.querySelectorAll('kbd'))
      kbd.classList.add('reset', 'instruction');
  }

  return (
    <div className='column expanded padded-horizontally'>
      <div className='expanded'>
        {contents[page()]}
      </div>
      <div className='row copy full-width'>
        <div className='expanded'>
          <Arrow page={page()} setPage={setPage} step={-1}>{'<-'}</Arrow>
          {` ${page() + 1}/${contents.length} `}
          <Arrow page={page()} setPage={setPage} step={1}>{'->'}</Arrow>
        </div>
        <a className='spaced-after' href='https://github.com/kofigumbs/typebeat'>GitHub</a>
        <a className='spaced-after' href='https://twitter.com/kofigumbs'>Twitter</a>
        <a className='spaced-after' href='https://instagram.com/kofigumbs'>IG</a>
      </div>
    </div>
  );
};

export default () => {
  const [appEvent, setAppEvent] = createSignal({});

  let ref;
  onMount(() => {
    const rows = Array.from(ref.children);
    const resize = () => {
      const left = Math.min(...rows.map(el => el.offsetLeft));
      const right = Math.max(...rows.map(el => el.offsetLeft + el.offsetWidth));
      const scale = Math.min(1, ref.parentElement.offsetWidth / (right - left));
      const margin = `${-.5 * ref.offsetHeight * (1 - scale)}px`;
      ref.style.transform = `scale(${scale})`;
      ref.style.marginTop = ref.style.marginBottom = margin;
    };
    resize();
    createEventListener(window, 'resize', resize);
  });

  return (
    <>
      <header className='copy full-width padded-horizontally'>
        <h1>Typebeat</h1> -- Play with music, quickly. Sampler + synth + sequencer.
      </header>
      <div ref={ref} className='app'>
        <App
          init={(state) => createEffect(() => setAppEvent({ modifier: state.modifier }))}
          send={(method, data) => {
            lib.then(lib => lib.send(method, data));
            setAppEvent({ [method]: data });
          }}
          onChange={(callback) => lib.then(lib => lib.onChange(callback))}
        />
      </div>
      <Guide appEvent={appEvent()} />
    </>
  );
};
