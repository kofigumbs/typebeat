import solidPlugin from 'vite-plugin-solid';
import { description } from '../package.json';

const meta = `
  <!-- Primary Meta Tags -->
  <title>Typebeat</title>
  <meta name="title" content="Typebeat">
  <meta name="description" content="${description}">

  <!-- Open Graph / Facebook -->
  <meta property="og:type" content="website">
  <meta property="og:url" content="https://typebeat.xyz">
  <meta property="og:title" content="Typebeat">
  <meta property="og:description" content="${description}">
  <meta property="og:image" content="https://typebeat.xyz/header-1200x628.png">

  <!-- Twitter -->
  <meta property="twitter:card" content="summary_large_image">
  <meta property="twitter:url" content="https://typebeat.xyz">
  <meta property="twitter:title" content="Typebeat">
  <meta property="twitter:description" content="${description}">
  <meta property="twitter:image" content="https://typebeat.xyz/header-1200x628.png">
`;

export default {
  plugins: [solidPlugin(), {
    name: 'insert-meta-tags',
    transformIndexHtml: html => html.replace('<head>', `<head>${meta}`),
  }],
  build: { sourcemap: true },
};
