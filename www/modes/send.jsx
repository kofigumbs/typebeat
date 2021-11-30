import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'F';

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'main',   commands: Commands.nudge('activeTrack', 'main'  ) },
  { cap: 'U', label: 'pan',    commands: Commands.nudge('activeTrack', 'pan'   ) },
  { cap: 'I', label: 'echo',   commands: Commands.nudge('activeTrack', 'echo'  ) },
  { cap: 'O', label: 'reverb', commands: Commands.nudge('activeTrack', 'reverb') },
  { cap: 'P', label: 'drive',  commands: Commands.nudge('activeTrack', 'drive' ) }
)

const Rect = props => {
  const s = createMemo(() => 8 + 16*props.state.activeTrack.main/50);
  const x = createMemo(() => 48 - s()/2 + props.state.activeTrack.pan);
  const y = createMemo(() => 23 - s()/2);
  const r = createMemo(() => `${props.state.activeTrack.reverb/2}%`);
  const spacing = createMemo(() => props.state.activeTrack.echo/4);
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
  <svg xmlns='http://www.w3.org/2000/svg'>
    <Rect i={0} {...props} />
    <Rect i={1} {...props} />
    <Rect i={2} {...props} />
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>SEND</b> mode controls how much sound is routed to each destination mix.
      By default, tracks only output to the <b>main</b> mix.
      {' '}<b>SEND</b> mode is per-track, and it only controls the amount of sound routed to the effect.
      To change the actual effect characteristics, use <b>RETURN</b> mode.
    </Block>
  </>
);
