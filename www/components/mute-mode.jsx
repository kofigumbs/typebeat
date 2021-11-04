import Actions from '../actions';
import { Grid, layout } from './track-mode';

export const cap = 'B';

export const actions = Actions.all({
  label: (state, i) => state.tracks[i].muted ? '██' : '--',
  onDown: (state, i) => state.send('muted', i),
});

const Mute = props => (
  <rect {...layout(props.x, props.y)} style={{
    '--key_fill': props.state.tracks[props.id].muted ? 'var(--dark)' : 'var(--key_background)',
  }} />
);

export const Visual = props => (
  <Grid component={Mute} {...props} />
);
