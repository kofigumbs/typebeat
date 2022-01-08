import { createMemo } from 'solid-js';

import Commands from '../commands';

export const cap = 'W';

export const commands = Commands.tabbed(
  { cap: 'Y', label: 'source', commands: Commands.tabbed(
    { cap: 'N', label: 'type', commands: Commands.select(
      'activeTrack',
      'sampleType',
      ['file', 'live =>', 'live .=', 'live |>']
    )},
    { cap: 'M', label: 'level', commands: Commands.nudge('activeTrack', 'sampleLevel') },
    { cap: ',', label: 'detune', commands: Commands.nudge('activeTrack', 'sampleDetune') },
    { cap: '.', label: 'dir.', commands: Commands.select('activeTrack', 'direction', ['forward', 'reverse'])},
  )},
);

const Waveform = props => {
  const s = createMemo(() => props.state.activeTrack[`waveform${props.i}`]/5 + 1);
  return (
    <path d={`M ${props.i*4 + 3} ${23 - s()} v ${s()*2}`} stroke-width='2' />
  );
};

export const Visual = props => {
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <For each={Array.from({ length: 24 })}>
        {(_, i) => <Waveform i={i()} {...props} />}
      </For>
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>SAMPLE</b> mode controls the track's sample oscillator, which can either play audio from a file or record input from the mic.
      Each track starts with a unique file sample at full volume.
      You can use <b>level</b> and <b>detune</b> to adjust its tuning.
    </Block>
    <Block>
      The default type, <b>file</b>, streams audio from the sample pack distributed with Typebeat.
      The three <b>live</b> types define the workflow for live-sampling: <b>live stream (=>)</b>, <b>live record (.=)</b>, and <b>live playback (|>)</b>.
      Live stream passes the mic input directly to the Typebeat effect chain, which is great for vocals or previewing a live-sample.
      Live record will also stream the mic input, but it will also write audio to the per-track sample memory.
      Finally, live playback will stream from the per-track memory, replaying whatever audio was live-sampled.
      Each track has 60 seconds of live-sample memory for recording/playback.
    </Block>
    <Block className='bullet'>
      Try recording your own sample with <kbd>K</kbd> and replaying it with <kbd>L</kbd>.
    </Block>
  </>
);
