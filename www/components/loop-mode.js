import bind from '../bind';

export const cap = 'S';

const maxResolution = 512;
const viewStart = async proxy => await proxy.pageStart / maxResolution / await proxy.resolution;

export const actions = (local, proxy, set) => new Map([
  ...bind.group('YUHJL;', i => ({
    label: () => ['bars -', 'bars +','zoom -', 'page -', 'page +', 'zoom +'][i],
    onDown: () => set(...[['length', -1], ['length', 1], ['zoomOut'], ['page', -1], ['page', 1], ['zoomIn']][i]),
  })),
  ...bind.group('NM,.', i => ({
    label: async () => {
      const n = ((await viewStart(proxy) + i) % await proxy.resolution) + 1;
      switch (await proxy[`view ${i}`]) {
        case 0: return '';
        case 1: return `${n}/${await proxy.resolution}`;
        case 2: return `${n}█${await proxy.resolution}`;
        case 3: return `${n}░${await proxy.resolution}`;
      }
    },
    onDown: () => set('sequence', i),
  })),
  ['P', bind.one({ label: () => 'clear', title: () => proxy.canClear, onDown: () => set('clear') }) ],
  ['K', bind.title(async () => `bar ${((await viewStart(proxy) / await proxy.resolution)|0) + 1}/${await proxy.length / maxResolution}`) ],
]);
