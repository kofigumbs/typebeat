/* 
 * Parse guide markdown for content and checks (encoded like `key=value`)
 */
const guide = () => {
  const horizontalRules = /(?:\n+)---(?:\n+)/;
  const trailingNewlinesAndCodeFenceChecks = /\n+(?:`.+\n*)$/;
  const paragraphBreaks = /\n(?:\n+)/;
  const codeFenceChecks = /`([^=]+)=([^`]+)`/g;
  return {
    name: 'transform-guide',
    transform(src, path) {
      if (!path.endsWith('/GUIDE.md'))
        return;
      const sections = src.split(horizontalRules).map(markup => {
        const content = markup
          .replace(trailingNewlinesAndCodeFenceChecks, '')
          .split(paragraphBreaks)
          .map(x => x.startsWith('<') ? x : `<p>${x}</p>`)
          .join('');
        const checks = [];
        for (let match of markup.matchAll(codeFenceChecks))
          checks.push([match[1], match[2]]);
        return { content, checks };
      });
      return {
        code: `export default ${JSON.stringify(sections)}`,
        map: null,
      };
    },
  };
};

export default {
  plugins: [guide()],
  hmr: false,
  build: { sourcemap: true },
};
