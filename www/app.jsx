import { createEffect, createMemo, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';

import 'firacode';
import Tare from 'tare';
import { pulse, type } from './animations';
import './index.css';

/*
 * Map of caps to mode names/modules
 */

const modeImports = import.meta.globEager('./components/*-mode.js*');
const modes = new Map();
const basename = /\/(\w+)-mode\./;
for (let [path, module] of Object.entries(modeImports)) {
  const name = path.match(basename)[1];
  modes.set(module.cap, { name, ...module });
}


/*
 * Event handlers
 */

const capsByEventCode = new Map([
  ['Semicolon', ';'], ['Comma', ','], ['Period', '.'], ['Slash', '/'],
  ...Array.from('QWERTYUIOPASDFGHJKLZXCVBNM', cap => [`Key${cap}`, cap]),
]);

const handleKeyboardEvent = callback => event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey)
    return;
  const cap = capsByEventCode.get(event.code);
  if (!cap)
    return
  event.preventDefault();
  if (!event.repeat)
    callback(cap);
};

const handlePointerEvent = callback => event => {
  if (event.button)
    return; // don't hijack right-click
  event.preventDefault();
  callback(event.target.dataset.cap);
};


/*
 * Components
 */

const Key = props => (
  <button
    className={`key ${props.className}`}
    classList={props.classList}
    data-cap={props.cap}
    onPointerDown={handlePointerEvent(props.onCapDown)}
    onPointerUp={handlePointerEvent(props.onCapUp)}
  >
    {props.children}
  </button>
);

const Mode = props => {
  const mode = modes.get(props.cap);
  const Visual = mode.Visual || (() => '');
  return (
    <Key className='mode' classList={{ active: props.cap === props.state.modifier }} {...props}>
      <div className='label' innerHTML={Tare.html(mode.name)} />
      <div className='visual'>
        <Show when={props.state.song}>
          <Visual state={props.state} />
        </Show>
      </div>
    </Key>
  );
};

const Action = props => {
  const [label, setLabel] = createSignal();
  const action = createMemo(() => props.state.actions.get(props.cap));
  const cache = type.cache();
  createEffect(oldLabel => {
    const newLabel = action()?.label(props.state)?.toString();
    type(newLabel, oldLabel, setLabel, cache);
    return newLabel;
  });
  return (
    <Key className='action' {...props}>
      <div
        className='label'
        classList={{ title: !!action()?.title(props.state) }}
        textContent={label()}
      />
    </Key>
  );
};

const Grid = props => (
  <For each={props.rows}>
    {row => (
      <div class='row'>
        <For each={row}>{props.children}</For>
      </div>
    )}
  </For>
);

export default props => {
  const [state, setState] = createStore({
    send: props.send,
    get actions() {
      return modes.get(this.modifier).actions;
    },
    get activeTrack() {
      return this.tracks[this.song.activeTrack];
    },
  });

  props.dump.then(setState);
  props.onChange(change => setState(...change));

  const onCapDown = cap => {
    if (cap === state.modifier)
      setState({ modifier: undefined });
    else if (modes.has(cap))
      setState({ modifier: cap });
    else {
      state.actions.get(cap)?.onDown?.(state);
      pulse(document.querySelector(`[data-cap="${cap}"]`));
    }
  };
  const onCapUp = cap => {
    state.actions.get(cap)?.onUp(state);
  };
  document.addEventListener('keydown', handleKeyboardEvent(onCapDown));
  document.addEventListener('keyup', handleKeyboardEvent(onCapUp));

  return (
    <Grid rows={['QWERTYUIOP', 'ASDFGHJKL;', 'ZXCVBNM,./']}>
      {cap => {
        const Component = modes.has(cap) ? Mode : Action;
        return <Component {...{ cap, state, onCapDown, onCapUp }} />
      }}
    </Grid>
  );
};
