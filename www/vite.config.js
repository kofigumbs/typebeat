import elmPlugin from 'vite-plugin-elm';

export default {
  build: { sourcemap: true },
  hmr: false,
  plugins: [elmPlugin()],
};
