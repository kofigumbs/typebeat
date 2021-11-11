const P = props => (
  <p className='copy full-width'>{props.children}</p>
);

const Kbd = props => (
  <span className='kbd'>{props.children}</span>
);

export default [
  { until: { code: 'Space' }, content: (
    <>
      <P>
        Get the full app for <a download href='/download/macos/Typebeat_0.1.0_x64.dmg'>macOS</a>,
        or press <Kbd>SPACE</Kbd> to start the online guide.
      </P>
    </>
  )},
  { until: { auditionDown: 7 }, content: (
    <>
      <P>
        Let's start by triggering a sample.
        Typebeat's controls are laid out in two halves.
        By default, any key on the right half of your keyboard triggers a sample.
      </P>
      <P>
        Try pressing <Kbd>K</Kbd> to trigger a clap.
      </P>
    </>
  )},
  { until: { auditionDown: 0 }, content: (
    <>
      <P>
        OK! Now try pressing <Kbd>N</Kbd> to trigger a kick drum.
      </P>
    </>
  )},
  { until: { modifier: 'Z' }, content: (
    <>
      <P>
        Congratulations, you now know the foundations of house music.
        In fact, you could make a simple beat by rhythmically typing this sequence:
        <Kbd>N</Kbd> <Kbd>Y</Kbd> <Kbd>K</Kbd> <Kbd>Y</Kbd>.
      </P>
      <P>
        Manually triggering sounds is great for jamming, but eventually you'll want to record your sequence.
        For that, we'll need to understand modes: or the left half of your keyboard.
        Press <Kbd>Z</Kbd> to enter <b>SONG</b> mode.
      </P>
    </>
  )},
  { until: { playing: 0 }, content: (
    <>
      <P>
        Once you enter a mode, the right-hand keys take on a different meaning.
        That's the main idea behind Typebeat's controls:
      </P>
      <ol>
        <li>Keys on the left select modes</li>
        <li>Keys on the right trigger actions</li>
      </ol>
      <P>
        Now that you're in <b>SONG</b> mode, press <Kbd>N</Kbd> to play the demo song.
      </P>
    </>
  )},
  { until: { never: [] }, content: (
    <>
      <P>
        That's all I have so far -- more coming soon!
      </P>
    </>
  )},
];
