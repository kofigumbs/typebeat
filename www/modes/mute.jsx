import Actions from '../actions';
import { Grid, layout } from './track';

export const cap = 'B';

export const actions = Actions.all({
  label: (state, i) => state.tracks[i].muted ? '██' : '--',
  onDown: (state, i) => state.send('muted', i),
});

const Mute = props => (
  <rect {...layout(props.x, props.y)} classList={{ dark: props.state.tracks[props.id].muted }} />
);

export const Visual = props => (
  <Grid component={Mute} {...props} />
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>MUTE</b> mode lets you silence the loop of any track.
      You'd mainly use this in a performance setting to dynamically bring different patterns in and out.
      The key mappings are the sames as those of <b>AUDITION</b> mode and <b>TRACK</b> mode.
    </Block>
  </>
);
