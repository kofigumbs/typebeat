import Actions from '../actions';

export const cap = undefined;

export const actions = Actions.all({
  onDown: (state, i) => state.send('auditionDown', i),
  onUp: (state, i) => state.send('auditionUp', i),
});

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>AUDITION</b> mode lets you trigger sounds on any track.
      Each key on the right corresponds to a track.
      {' '}<b>AUDITION</b> is the default mode if no key on the left is selected.
    </Block>
    <Block className='bullet'>
      Try playing a house beat by typing the following sequence: <kbd>N</kbd><kbd>Y</kbd><kbd>K</kbd><kbd>Y</kbd>.
    </Block>
    <Block className='bullet'>
      When you're done jamming, try pressing <kbd>Z</kbd> to enter <b>SONG</b> mode.
    </Block>
  </>
);
