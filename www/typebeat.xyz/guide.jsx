export default [
  { until: { code: 'Space' }, content: (
    <>
      <p>
        Press <button><kbd>SPACE</kbd></button> to start the guide.
      </p>
    </>
  )},
  { until: { method: 'auditionDown', data: 7 }, content: (
    <>
      <p>
        Let's start by triggering a sample.
        Typebeat's controls are laid out in two halves.
        By default, any key on the right half of your keyboard triggers a sample.
      </p>
      <p>
        Try pressing <kbd>K</kbd> to trigger a clap.
      </p>
    </>
  )},
  { until: { method: 'auditionDown', data: 0 }, content: (
    <>
      <p>
        OK! Now try pressing <kbd>N</kbd> to trigger a kick drum.
      </p>
    </>
  )},
  { until: { modifier: 'T' }, content: (
    <>
      <p>
        Congratulations, you now know the foundations of house music.
      </p>
      <p>
        All the keys on the right work this way, so you could make a simple beat by rhythmically typing this sequence:
        <kbd>N</kbd> <kbd>Y</kbd> <kbd>K</kbd> <kbd>Y</kbd>.
      </p>
      <p>
        Next, we're going to talk about modes: or the left half of your keyboard.
        Let's say that kick (<kbd>N</kbd>) is just too high pitch for your taste.
        We can go into <b>NOTE</b> mode and change that.
      </p>
      <p>
        Press <kbd>T</kbd> to enter <b>NOTE</b> mode.
      </p>
    </>
  )},
  { until: { method: 'activeTrack', data: 7 }, content: (
    <>
      <p>
        In <b>NOTE</b> mode, every key on the right triggers a different note.
        This illustrates the main idea behind Typebeat's controls:
      </p>
      <ol>
        <li>Keys on the left select modes</li>
        <li>Keys on the right trigger actions</li>
      </ol>
      <p>
        The default mode lets you preview any of your tracks.
        <b>NOTE</b> mode lets you play different pitches on one of your tracks: whichever one is active.
        <kbd>Q</kbd> is a special mode that lets you select the active track.
        Let's try that now.
      </p>
      <p>
        Press <kbd>Q</kbd> to enter the track select mode, and then press <kbd>K</kbd> to select the clap track.
      </p>
    </>
  )},
  { until: { never: [] }, content: (
    <>
      <p>
        That's all I have so far -- more coming soon!
      </p>
    </>
  )},
];
