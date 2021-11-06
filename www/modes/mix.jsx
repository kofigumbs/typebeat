import { createMemo } from 'solid-js';

import Actions from '../actions';

export const cap = 'G';

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'main',   actions: Actions.nudge('activeTrack', 'main'  ) },
  { cap: 'U', label: 'pan',    actions: Actions.nudge('activeTrack', 'pan'   ) },
  { cap: 'I', label: 'echo',   actions: Actions.nudge('activeTrack', 'echo'  ) },
  { cap: 'O', label: 'reverb', actions: Actions.nudge('activeTrack', 'reverb') },
  { cap: 'P', label: 'drive',  actions: Actions.nudge('activeTrack', 'drive' ) }
)

const Rect = props => {
  const s = createMemo(() => 24 * props.state.activeTrack.main / 50);
  const x = createMemo(() => 48 - s()/2 + props.state.activeTrack.pan);
  const y = createMemo(() => 23 - s()/2);
  const r = createMemo(() => `${props.state.activeTrack.reverb / 2}%`);
  const spacing = createMemo(() => props.state.activeTrack.echo / 4);
  const strokeWidth = createMemo(() => props.state.activeTrack.drive + 2);
  return (
    <rect
      x={x() + (props.i-1)*spacing()}
      y={y() + (props.i-1)*spacing()}
      rx={r()}
      ry={r()}
      width={s()}
      height={s()}
      stroke-width={strokeWidth()}
    />
  );
};

export const Visual = props => (
  <svg xmlns="http://www.w3.org/2000/svg">
    <Rect i={0} {...props} />
    <Rect i={1} {...props} />
    <Rect i={2} {...props} />
  </svg>
);
