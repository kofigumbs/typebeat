import bind from '../bind';
import pulse from '../pulse';
import TrackGrid from './track-grid';

export const cap = 'Q';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => i === await proxy.activeTrack ? 'active' : '',
    title: async () => !await proxy.playing,
    onDown: () => set('activeTrack', i),
  })),
]);

customElements.define('track-mode', class extends TrackGrid {
  async sync({ proxy }) {
    const activeTrack = await proxy.activeTrack;
    this._tracks.forEach(async (track, i) => {
      if (await proxy[`recent ${i}`])
        pulse(track);
      track.classList.toggle('active', i === activeTrack);
    });
  }
});
