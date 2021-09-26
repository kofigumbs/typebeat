Press <kbd>N</kbd> to start the guide.

`method=auditionDown` `data=7`
---

You just triggered a sample!
Typebeat's controls are laid out in two halves.
By default, any key on the right half of your keyboard triggers a sample.

Try pressing <kbd>K</kbd> to trigger a clap.

`method=auditionDown` `data=7`

---

OK! Now try pressing <kbd>N</kbd> to trigger a kick drum.

`method=auditionDown` `data=0`

---

Congratulations, you now know the foundations of house music.

All the keys on the right work this way, so you could make a simple beat by rhythmically typing this sequence:
<kbd>N</kbd> <kbd>Y</kbd> <kbd>K</kbd> <kbd>Y</kbd>.

Next, we're going to talk about modes: or the left half of your keyboard.
Let's say that kick (<kbd>N</kbd>) is just too high pitch for your taste.
We can go into <b>NOTE</b> mode and change that.

Press <kbd>T</kbd> to enter <b>NOTE</b> mode.

`method=activeKey`

---

In <b>NOTE</b> mode, every key on the right triggers a different note.
This illustrates the main idea behind Typebeat's controls:

<ol>
  <li>Keys on the left select modes</li>
  <li>Keys on the right trigger actions</li>
</ol>

The default mode lets you preview any of your tracks.
<b>NOTE</b> mode lets you play different pitches on one of your tracks: whichever one is active.
<kbd>Q</kbd> is a special mode that lets you select the active track.
Let's try that now.

Press <kbd>Q</kbd> to enter the track select mode, and then press <kbd>K</kbd> to select the clap track.

`method=activeTrack` `data=7`

---

You can always tell which track is active by the highlighted box in the top-left corner.
Now that the clap (<kbd>K</kbd>) is active, <b>NOTE</b> mode will change its pitch.

Go back to <b>NOTE</b> mode and play a different clap pitch.

`method=noteDown`

---

Two final remarks about <b>NOTE</b> mode.
First, the grid is ordered southwest-to-northeast, where <kbd>N</kbd> is the lowest pitch and <kbd>P</kbd> is the highest.

Second, whichever note you select last becomes the active note, as indicated by the underline.
The active note is relevant for other modes like <b>LOOP</b>, which we'll discuss next.

Press <kbd>S</kbd> to enter <b>LOOP</b> mode.

`method=viewStart`

---

<b>LOOP</b> mode is Typebeat's step sequencer.
You can use it to program notes with the precise timing.
If you're unfamiliar with sequencers, it's easiest to learn by doing.

Press <kbd>N</kbd> to sequence a note on the first step.

`method=sequence` `data=0`

---

The █ means you've placed a note on that step, but you won't hear anything until you play the beat.
You control playback from <b>BEAT</b> mode.

Press <kbd>A</kbd> to enter <b>BEAT</b> mode, and then press <kbd>N</kbd> to play.

`context=set` `method=play`

---

That's all I have so far -- more coming soon!

`done=true`
