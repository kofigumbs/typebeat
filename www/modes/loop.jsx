import Actions from '../actions';

export const cap = 'S';

export const actions = new Map();

// export const actions = (state) => new Map([
//   ...bind.group('YUHJL;', i => ({
//     label: () => ['bars -', 'bars +','zoom -', 'page -', 'page +', 'zoom +'][i],
//     onDown: () => state.send(...[['length', -1], ['length', 1], ['zoomOut'], ['page', -1], ['page', 1], ['zoomIn']][i]),
//   })),
//   ...bind.group('NM,.', i => ({
//     label: () => {
//       const n = ((state.activeTrack('viewStart') + i) % state.activeTrack('resolution')) + 1;
//       switch (state.activeTrack(`view${i}`)) {
//         case 0: return '';
//         case 1: return `${n}/${state.activeTrack('resolution')}`;
//         case 2: return `${n}█${state.activeTrack('resolution')}`;
//         case 3: return `${n}░${state.activeTrack('resolution')}`;
//       }
//     },
//     onDown: () => state.send('sequence', i),
//   })),
//   ['P', bind.one({
//     label: () => 'clear',
//     title: () => state.activeTrack('canClear'),
//     onDown: () => state.send('clear', 0),
//   })],
//   ['K', bind.title(() => {
//     const bar = ((state.activeTrack('viewStart') / state.activeTrack('resolution'))|0) + 1;
//     return `bar ${bar}/${state.activeTrack('bars')}`
//   })],
// ]);
