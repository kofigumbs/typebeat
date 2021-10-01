/* 
 * Parse GUIDE.md for content and checks (encoded as `key=value`)
 */
const transformGuide = () => {
  const horizontalRules = /(?:\n+)---(?:\n+)/;
  const trailingNewlinesAndCodeFenceChecks = /\n+(?:`.+\n*)$/;
  const paragraphBreaks = /\n(?:\n+)/;
  const codeFenceChecks = /`([^=]+)=([^`]+)`/g;
  const blockElement = /^<(?:ul|ol)>/
  return {
    name: 'transform-guide',
    transform(src, path) {
      if (!path.endsWith('/GUIDE.md'))
        return;
      const sections = src.split(horizontalRules).map(markup => ({
        content: markup
          .replace(trailingNewlinesAndCodeFenceChecks, '')
          .split(paragraphBreaks)
          .map(x => x.match(blockElement) ? x : `<p>${x}</p>`)
          .join(''),
        checks: Array.from(markup.matchAll(codeFenceChecks), match => (
          [match[1], match[2]]
        )),
      }));
      return {
        code: `export default ${JSON.stringify(sections)}`,
        map: null,
      };
    },
  };
};

export default {
  plugins: [transformGuide()],
  hmr: false,
  build: { sourcemap: true },
};
