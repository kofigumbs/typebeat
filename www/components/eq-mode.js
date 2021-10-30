import Actions from '../actions';

export const cap = 'F';

const subtabs = (band) => Actions.tabbed(
  { cap: 'N', label: 'freq.', actions: Actions.nudge('activeTrack', `${band}Freq`) },
  { cap: 'M', label: 'res.',  actions: Actions.nudge('activeTrack', `${band}Res` ) }
);

export const actions = Actions.tabbed(
  { cap: 'Y', label: 'low',  actions: subtabs('low') },
  { cap: 'U', label: 'mid',  actions: subtabs('mid') },
  { cap: 'I', label: 'high', actions: subtabs('high') }
);

// const bands = ['low', 'band 1', 'band 2', 'band 3', 'high'];
//
// customElements.define('eq-mode', class extends HTMLElement {
//   all = (state, filter) => bands.map(x => state.activeTrack(bind.join(x, filter));
//   normalize = (freq, i) => (i === 0 || i === 4) ? freq/50 : (freq + 25)/50;
//
//   connectedCallback() {
//     this.innerHTML = `
//       <svg xmlns="http://www.w3.org/2000/svg">
//         <path fill="none" stroke-width="2"></path>
//       </svg>
//     `;
//     this._path = this.querySelector('path');
//   }
//
//   sync(state) {
//     const margin = 3;
//     const bandWidth = 90/5;
//     const bandHeight = 20;
//     const [lowX, band1X, band2X, band3X, highX] =
//       this.all(state, 'freq.')
//       .map((freq, i) => margin + bandWidth*i + bandWidth*this.normalize(freq, i));
//     const [lowY, band1Y, band2Y, band3Y, highY] =
//       this.all(state, 'res.')
//       .map((res, i) => margin + bandHeight*(1 + res/-50));
//     const mid1 = (band1X + lowX)   / 2;
//     const mid2 = (band2X + band1X) / 2;
//     const mid3 = (band3X + band2X) / 2;
//     const mid4 = (highX  + band3X) / 2;
//     this._path.setAttribute('d', `
//       M ${lowX} ${lowY}
//       C ${mid1} ${lowY}   ${mid1} ${band1Y} ${band1X} ${band1Y}
//       C ${mid2} ${band1Y} ${mid2} ${band2Y} ${band2X} ${band2Y}
//       C ${mid3} ${band2Y} ${mid3} ${band3Y} ${band3X} ${band3Y}
//       C ${mid4} ${band3Y} ${mid4} ${highY}  ${highX}  ${highY}
//     `);
//   }
// });
