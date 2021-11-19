export default [
  { until: 'keypress', is: 'Space', render: (props) => (
    <>
      <p>
        Get the full app for <a download href='/download/macos/Typebeat_0.1.0_x64.dmg'>macOS</a>,
        or press <button className='reset' onClick={() => props.advance({ keypress: 'Space' })}><kbd>SPACE</kbd></button> to start the online guide.
      </p>
    </>
  )},
  { until: 'auditionDown', is: 7, render: () => (
    <>
      <p>
        Let's begin by triggering a sample.
        Typebeat's controls are laid out in two halves.
        By default, any key on the right half of your keyboard triggers a sample.
      </p>
      <p>
        Try pressing <kbd>K</kbd> to trigger a clap.
      </p>
    </>
  )},
  { until: 'auditionDown', is: 0, render: () => (
    <>
      <p>
        OK! Now try pressing <kbd>N</kbd> to trigger a kick drum.
      </p>
    </>
  )},
  { until: 'modifier', is: 'Z', render: () => (
    <>
      <p>
        Congratulations, you now know the foundations of house music.
        In fact, you could make a simple beat by rhythmically typing this sequence:
        <kbd>N</kbd><kbd>Y</kbd><kbd>K</kbd><kbd>Y</kbd>.
      </p>
      <p>
        Manually triggering sounds is great for jamming, but eventually you'll want to record your sequence.
        For that, we'll need to understand modes: or the left half of your keyboard.
      </p>
      <p>
        Press <kbd>Z</kbd> to enter <b>SONG</b> mode.
      </p>
    </>
  )},
  { until: 'playing', is: 0, render: () => (
    <>
      <p>
        Once you enter a mode, the right-hand keys take on a different meaning.
        That's the main idea behind Typebeat's controls:
      </p>
      <ol>
        <li>Keys on the left select modes</li>
        <li>Keys on the right trigger actions</li>
      </ol>
      <p>
        Now that you're in <b>SONG</b> mode, press <kbd>N</kbd> to play the demo beat.
      </p>
    </>
  )},
  { until: 'auditionDown', is: 0, render: () => (
    <>
      <p>
        The timeline is playing!
        There's a simple baseline and some percussion to get you started, but this beat needs some kick.
      </p>
      <p>
        Press <kbd>Z</kbd> again to exit <b>SONG</b> mode.
        Then try playing along with the kick drum (<kbd>N</kbd>) to find a sequence you like.
      </p>
    </>
  )},
  { until: 'recording', is: 0, render: () => (
    <>
      <p>
        Once you find a sequence that you like, you'll want to record it so that it loops automatically.
        Press <kbd>Z</kbd> to re-enter <b>SONG</b> mode, then press <kbd>M</kbd> to start recording.
      </p>
    </>
  )},
  { until: 'auditionDown', is: 0, render: () => (
    <>
      <p>
        You're recording now -- any sound you trigger will be saved to the loop.
        You might recognize the drill by now: press <kbd>Z</kbd> to exit <b>SONG</b> mode, then play/record your kick (<kbd>N</kbd>) sequence.
      </p>
      <p>
        (As you're getting started, Typebeat might feel a bit repetitive: like re-learning to type.
        But once you're familiar with the layout, it's really, really fast way to create music.
        You can reach every control from the 30 keyboard keys in front of you, so you'll never have to click through menus to find a setting.
        It's very fun and freeing once you're used to it: just type on your keyboard, and a beat comes out!)
      </p>
    </>
  )},
  { until: 'modifier', is: 'A', render: () => (
    <>
      <p>
        If you didn't nail the timing on your recording, you can always edit it.
        Press <kbd>A</kbd> to enter <b>LOOP</b> mode.
      </p>
    </>
  )},
  { until: 'sequence', is: [0, 1, 2, 3], render: () => (
    <>
      <p>
        Typebeat tracks all start with a 16-step loop.
        In <b>LOOP</b> mode, you can see see 4 steps at a time.
        You can use <kbd>J</kbd> and <kbd>L</kbd> to move your view across the 16 steps in your loop.
      </p>
      <p>
        The keys on the third row represent the steps themselves.
        If the key is marked with █, then it will trigger a sound on that step.
        Pressing any of those keys will toggle the trigger on that step, which makes it an alternative to recording your sequence live.
      </p>
      <p>
        Press <kbd>N</kbd>, <kbd>M</kbd>, <kbd>,</kbd>, or <kbd>.</kbd> to edit a recorded step.
      </p>
    </>
  )},
  { until: 'playing', is: 0, render: () => (
    <>
      <p>
        This process of recording and tweaking small loops is the center of the Typebeat workflow.
        I recommend going track-by-track recording parts for each instrument, building up a beat layer-by-layer.
        Typically, I'll keep the beat playing the entire time, but for this guide let's pause and cover a few more modes.
      </p>
      <p>
        Press <kbd>Z</kbd> to enter <b>SONG</b> mode, then press <kbd>N</kbd> to stop the beat.
      </p>
    </>
  )},
  { is: 'end', render: () => (
    <>
      <p>
        That's all I have so far -- more coming soon!
      </p>
    </>
  )},
];
