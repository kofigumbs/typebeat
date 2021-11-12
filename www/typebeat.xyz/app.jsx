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
    window.addEventListener('beforeunload', () => {
      if (started)
        ccall('typebeat_stop', null)
    });

    const onChange = callback => {
      callback([getJson('typebeat_dump')]);
      (function poll() {
        getJson('typebeat_changes').forEach(callback);
        requestIdleCallback(poll)
      })();
    };

    return { dump, send, onChange };
  });

const GuidePage = props => {
  const next = createMemo(() => props.page + props.step);
  const disabled = createMemo(() => next() < 0 || next() >= guide.length);
  return (
    <button
      className='spaced-after'
      classList={{ title: !disabled() }}
      disabled={disabled()}
      onClick={() => props.setPage(next())}
    >
      {props.children}
    </button>
  );
};

const Guide = props => {
  const [page, setPage] = createSignal(0);
  const advance = event => {
    for (let [key, value] of Object.entries(guide[page()].until))
      if (event[key] !== value)
        return;
    setPage(i => i + 1);
  };

  createEffect(() => advance(props.lastTask));
  createEffect(() => advance({ modifier: props.modifier }));
  createEventListener(document, 'keypress', event => advance({ code: event.code }));
  let ref;
  onMount(() => ref.querySelector('button').addEventListener('click', () => advance({ code: 'Space' })));

  return (
    <div className='column expanded padded-horizontally' ref={ref}>
      <div className='expanded'>
        {guide[page()].content}
      </div>
      <div className='copy full-width'>
        <GuidePage page={page()} setPage={setPage} step={-1}>Back</GuidePage>
        <GuidePage page={page()} setPage={setPage} step={1}>Next</GuidePage>
        <button className='title' onClick={() => props.setLabeled(x => !x)}>
          {props.labeled ? 'Hide' : 'Show'} labels
        </button>
      </div>
    </div>
  );
};

export default () => {
  const [labeled, setLabeled] = createSignal(true);
  const [lastTask, setLastTask] = createSignal({});
  const [modifier, setModifier] = createSignal();

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
        <h1>Typebeat</h1> - Make music using a familiar layout
      </header>
      <div ref={ref} className='mount' classList={{ labeled: labeled() }}>
        <App
          dump={lib.then(lib => lib.dump)}
          init={(state) => createEffect(() => setModifier(state.modifier))}
          send={(method, data) => {
            lib.then(lib => lib.send(method, data));
            setLastTask({ [method]: data });
          }}
          onChange={(callback) => lib.then(lib => lib.onChange(callback))}
        />
      </div>
      <Guide
        modifier={modifier()}
        lastTask={lastTask()}
        labeled={labeled()}
        setLabeled={setLabeled}
      />
    </>
  );
};
