import bind from '../bind';

export const cap = 'C';

export const actions = (state) => new Map([
  ...bind.oneOf('YUI', 'effect', ['reverb', 'echo', 'drive'], state),
  ...bind.nudge(
    () => state.song[bind.join(state.effect, state.effectControl)],
    i => state.send(bind.join(state.effect, state.effectControl), i)
  ),
  ...bind.oneOf('NM,', 'effectControl', ['gain', 'feed', 'space'], state),
]);
