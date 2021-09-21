// Parse guide markdown for content and checks (encoded like `key=value`)
const guide = () => {
  const horizontalRules = /(?:\n+)---(?:\n+)/;
  const trailingNewlinesAndCodeFenceChecks = /\n+(:?`.+\n*)$/;
  const paragraphBreaks = /\n(:?\n+)/;
  const codeFenceChecks = /`(\w+)=(\w+)`/g;
  return {
    name: 'transform-guide',
    transform(src, path) {
      if (!path.endsWith('/GUIDE.md'))
        return;
      const sections = src.split(horizontalRules).map(md => {
        const content = md
          .replace(trailingNewlinesAndCodeFenceChecks, '')
          .split(paragraphBreaks)
          .map(x => x.startsWith('<') ? x : `<p>${x}</p>`)
          .join('');
        const checks = [];
        for (let match of md.matchAll(codeFenceChecks))
          checks.push([match[1], match[2]]);
        return { content, checks };
      });
      return { code: `export default ${JSON.stringify(sections)}` };
    },
  };
};

export default {
  plugins: [guide()]
}
