import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'D';

const subtabs = (band) => Commands.tabbed(
  { cap: 'N', label: 'freq.', commands: Commands.nudge('activeTrack', `${band}Freq`) },
  { cap: 'M', label: 'res.',  commands: Commands.nudge('activeTrack', `${band}Res` ) }
);

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'low',  commands: subtabs('low') },
  { cap: 'U', label: 'mid',  commands: subtabs('mid') },
  { cap: 'I', label: 'high', commands: subtabs('high') }
);

const margin = 3;
const bandWidth = 90/3;
const bandHeight = 20;
const x = (freq, i) => margin + bandWidth*i + bandWidth*(i === 1 ? (freq + 25)/50 : freq/50);
const y = (res) => margin + bandHeight*(1 + res/-50);

export const Visual = props => {
  const commands = createMemo(() => {
    const lowX = x(props.state.activeTrack.lowFreq, 0);
    const midX = x(props.state.activeTrack.midFreq, 1);
    const highX = x(props.state.activeTrack.highFreq, 2);
    const lowY = y(props.state.activeTrack.lowRes);
    const midY = y(props.state.activeTrack.midRes);
    const highY = y(props.state.activeTrack.highRes);
    const join1 = (midX + lowX)  / 2;
    const join2 = (highX + midX) / 2;
    return `
      M ${lowX}  ${lowY}
      C ${join1} ${lowY} ${join1} ${midY}  ${midX}  ${midY}
      C ${join2} ${midY} ${join2} ${highY} ${highX} ${highY}
    `;
  });
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <path d={commands()} stroke-width='2'></path>
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
    </Block>
  </>
);
