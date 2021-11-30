export default [
  { until: 'keypress', is: 'Space', render: ({ Block, advance }) => (
    <>
      <Block>
        Want to try it first?
        Press <button className='reset' onClick={({ Block }) => props.advance({ keypress: 'Space' })}><kbd>SPACE</kbd></button> to start the online guide.
        The guide works best on desktop computers using Chrome or Firefox.
      </Block>
    </>
  )},
  { until: 'auditionDown', is: 7, render: ({ Block }) => (
    <>
      <Block>
        Let's begin by triggering a sample.
        Typebeat's controls are laid out in two halves.
        By default, any key on the right half of your keyboard triggers a sample.
      </Block>
      <Block>
        Try pressing <kbd>K</kbd> to trigger a clap.
      </Block>
    </>
  )},
  { until: 'auditionDown', is: 0, render: ({ Block }) => (
    <>
      <Block>
        OK! Now try pressing <kbd>N</kbd> to trigger a kick drum.
      </Block>
    </>
  )},
  { until: 'modifier', is: 'Z', render: ({ Block }) => (
    <>
      <Block>
        Congratulations, you now know the foundations of house music.
        In fact, you could make a simple beat by rhythmically typing this sequence:
        {' '}<kbd>N</kbd><kbd>Y</kbd><kbd>K</kbd><kbd>Y</kbd>.
      </Block>
      <Block>
        Manually triggering sounds is great for jamming, but eventually you'll want to record your sequence.
        For that, we'll need to understand modes: or the left half of your keyboard.
      </Block>
      <Block>
        Press <kbd>Z</kbd> to enter <b>SONG</b> mode.
      </Block>
    </>
  )},
  { until: 'playing', is: 0, render: ({ Block }) => (
    <>
      <Block>
        Once you enter a mode, the right-hand keys take on a different meaning.
        That's the main idea behind Typebeat's controls:
      </Block>
      <Block tagName='ol'>
        <li>Keys on the left select modes</li>
        <li>Keys on the right trigger commands</li>
      </Block>
      <Block>
        Now that you're in <b>SONG</b> mode, press <kbd>N</kbd> to play the demo beat.
      </Block>
    </>
  )},
  { until: 'recording', is: 0, render: ({ Block }) => (
    <>
      <Block>
        It's alive!
        There's a simple base and snare sequence to get you started, but this beat needs some kick.
      </Block>
      <Block>
        Press <kbd>M</kbd> to start recording.
      </Block>
    </>
  )},
  { until: 'modifier', is: undefined, render: ({ Block }) => (
    <>
      <Block>
        You're recording now -- any sound you trigger will be saved to the loop.
        Press <kbd>Z</kbd> to exit <b>SONG</b> mode and return to the default mode.
        (If you're wondering, the default mode is called <b>AUDITION</b> mode.)
      </Block>
    </>
  )},
  { until: 'auditionDown', is: 0, render: ({ Block }) => (
    <>
      <Block>
        Now let's add some kick to this beat.
        Remember -- every sound you trigger will be saved to the loop.
      </Block>
      <Block>
        Press <kbd>N</kbd> rhythmically to record a kick sequence.
      </Block>
    </>
  )},
  { until: 'modifier', is: 'A', render: ({ Block }) => (
    <>
      <Block>
        If you didn't nail the timing on your recording, you can always edit it.
        Press <kbd>A</kbd> to enter <b>LOOP</b> mode.
      </Block>
    </>
  )},
  { until: 'page', is: 1, render: ({ Block }) => (
    <>
      <Block>
        Typebeat tracks start with a 16-step loop.
        In <b>LOOP</b> mode, you can view 4 steps at a time.
        You can use <kbd>J</kbd> and <kbd>L</kbd> to move your view across the 16 steps in your loop.
      </Block>
      <Block>
        Press <kbd>L</kbd> to move through the loop and find the steps you recorded.
      </Block>
    </>
  )},
  { until: 'sequence', is: [0, 1, 2, 3], render: ({ Block }) => (
    <>
      <Block>
        Keys on the bottom row represent the steps themselves.
        If the key is marked with █, then that step will trigger a sound.
        Pressing any of those keys will toggle the trigger on that step, which makes it an alternative to recording your sequence live.
      </Block>
      <Block>
        Press <kbd>N</kbd>, <kbd>M</kbd>, <kbd>,</kbd>, or <kbd>.</kbd> to edit a recorded step.
      </Block>
    </>
  )},
  // { until: 'playing', is: 0, render: ({ Block }) => (
  //   <>
  //     <Block>
  //       This process of recording and tweaking small loops is the core of the Typebeat workflow.
  //       Once you're happy with one track, you can move on to the next and build up your beat layer-by-layer.
  //       Typically, I'll keep the beat playing the entire time, but for this guide let's pause and cover a few more modes.
  //     </Block>
  //     <Block>
  //       Press <kbd>Z</kbd> to enter <b>SONG</b> mode, then press <kbd>N</kbd> to stop the beat.
  //     </Block>
  //   </>
  // )},
  // { until: 'activeTrack', is: 8, render: ({ Block }) => (
  //   <>
  //     <Block>
  //       Next we'll discuss <b>TRACK</b> mode, which lets you select... tracks.
  //       Each track in Typebeat has its own sample, synth, and loop.
  //       Most other modes only affect the instrument on the active track.
  //     </Block>
  //     <Block>
  //       Press <kbd>Q</kbd> to enter <b>TRACK</b> mode, then press <kbd>L</kbd> to select the cowbell track.
  //     </Block>
  //   </>
  // )},
  // { until: 'modifier', is: 'T', render: ({ Block }) => (
  //   <>
  //     <Block>
  //       Earlier when we triggered sounds, we would always do so without a mode selected.
  //       That method gave us quick access to all of our tracks at once, but each track only used a single pitch.
  //       One way to make your beats more dynamic is to vary the pitch of your sounds.
  //     </Block>
  //     <Block>
  //       That's where <b>NOTE</b> mode comes in: press <kbd>T</kbd> to enter <b>NOTE</b> mode.
  //     </Block>
  //   </>
  // )},
  // { is: {}, render: (props) => (
  //   <>
  //     <Block>
  //       In <b>NOTE</b> mode, each key plays a different note on the active track.
  //       The lowest note is in the bottom-left corner, and the highest in the top-right.
  //       You can play "mary had a little lamb" on cowbell, by rhythmically typing this sequence:
  //       {' '}<kbd>P</kbd><kbd>O</kbd><kbd>I</kbd><kbd>O</kbd><kbd>P</kbd><kbd>P</kbd><kbd>P</kbd>.
  //     </Block>
  //     <Block>
  //       At this point, we've covered Typebeat's core modes, and you could make your own beats just using those.
  //       Give that a try!
  //       When you're ready to dig deeper, consider checking out <a href="https://github.com/kofigumbs/typebeat/blob/main/book">the book</a>.
  //     </Block>
  //     <Block>
  //       Happy typing!
  //     </Block>
  //   </>
  // )},
  { is: {}, render: ({ Block }) => (
    <>
      <Block>
        That's all I have so far -- more coming soon!
      </Block>
    </>
  )},
];
