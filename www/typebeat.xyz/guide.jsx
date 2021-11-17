export default [
  { until: { code: 'Space' }, show: (props) => (
    <>
      <p className={props.className}>
        Get the full app for <a download href='/download/macos/Typebeat_0.1.0_x64.dmg'>macOS</a>,
        or press <button onClick={() => props.advance({ code: 'Space' })}><kbd>SPACE</kbd></button> to start the online guide.
      </p>
    </>
  )},
  { until: { auditionDown: 7 }, show: (props) => (
    <>
      <p className={props.className}>
        Let's begin by triggering a sample.
        Typebeat's controls are laid out in two halves.
        By default, any key on the right half of your keyboard triggers a sample.
      </p>
      <p className={props.className}>
        Try pressing <kbd>K</kbd> to trigger a clap.
      </p>
    </>
  )},
  { until: { auditionDown: 0 }, show: (props) => (
    <>
      <p className={props.className}>
        OK! Now try pressing <kbd>N</kbd> to trigger a kick drum.
      </p>
    </>
  )},
  { until: { modifier: 'Z' }, show: (props) => (
    <>
      <p className={props.className}>
        Congratulations, you now know the foundations of house music.
        In fact, you could make a simple beat by rhythmically typing this sequence:
        <kbd>N</kbd><kbd>Y</kbd><kbd>K</kbd><kbd>Y</kbd>.
      </p>
      <p className={props.className}>
        Manually triggering sounds is great for jamming, but eventually you'll want to record your sequence.
        For that, we'll need to understand modes: or the left half of your keyboard.
        Press <kbd>Z</kbd> to enter <b>SONG</b> mode.
      </p>
    </>
  )},
  { until: { playing: 0 }, show: (props) => (
    <>
      <p className={props.className}>
        Once you enter a mode, the right-hand keys take on a different meaning.
        That's the main idea behind Typebeat's controls:
      </p>
      <ol className={props.className}>
        <li>Keys on the left select modes</li>
        <li>Keys on the right trigger actions</li>
      </ol>
      <p className={props.className}>
        Now that you're in <b>SONG</b> mode, press <kbd>N</kbd> to play the demo song.
      </p>
    </>
  )},
  { until: { forever: [] }, show: (props) => (
    <>
      <p className={props.className}>
        That's all I have so far -- more coming soon!
      </p>
    </>
  )},
];
