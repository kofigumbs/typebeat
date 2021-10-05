import bind from '../bind';
import TrackGrid from './track-grid';

export const cap = 'B';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => await proxy[`muted ${i}`] ? '██' : '--',
    onDown: () => set('muted', i),
  })),
]);

customElements.define('mute-mode', class extends TrackGrid {
  async sync({ proxy }) {
    this._tracks.forEach(async (track, i) => {
      track.classList.toggle('active', await proxy[`muted ${i}`]);
    });
  }
});
