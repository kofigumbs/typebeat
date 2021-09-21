Press <kbd>SPACE</kbd> to start the tutorial.

---

Let's start by triggering a sample.
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

You are now in <b>NOTE</b> mode, where every key on the right triggers a different note.
This illustrates the main idea behind Typebeat's controls:

<ol>
  <li>Keys on the left select modes</li>
  <li>Keys on the right trigger actions</li>
</ol>

The default mode lets you preview any of your tracks.
<b>NOTE</b> mode lets you play different pitches on one of your tracks: whichever one is active.
<kbd>Q</kbd> is a special mode that lets you determine which track is active.

Let's change the active track.
Press <kbd>Q</kbd> to enter the track select mode, and then press <kbd>K</kbd> to activate the clap track.

`method=activeTrack` `data=7`

---

That's all I have so far -- more coming soon!

`done=true`
