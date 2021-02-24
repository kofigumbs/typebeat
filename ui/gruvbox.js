/*
 * https://github.com/morhetz/gruvbox
 */

const withCss = (name, value) => {
  document.documentElement.style.setProperty(name, value)
  return value;
}

const bg0 = withCss("--bg0", "#fbf1c7");
const bg1 = withCss("--bg1", "#ebdbb2");
const bg2 = withCss("--bg2", "#d5c4a1");
const bg3 = withCss("--bg3", "#bdae93");
const bg4 = withCss("--bg4", "#a89984");
const bg0Hard = withCss("--bg0-hard", "#f9f5d7");
const bg0Soft = withCss("--bg0-soft", "#f2e5bc");
const fg0 = withCss("--fg0", "#282828");
const fg1 = withCss("--fg1", "#3c3836");
const fg2 = withCss("--fg2", "#504945");
const fg3 = withCss("--fg3", "#665c54");
const fg4 = withCss("--fg4", "#7c6f64");
const lightRed = withCss("--light-red", "#cc241d");
const lightGreen = withCss("--light-green", "#98971a");
const lightYellow = withCss("--light-yellow", "#d79921");
const lightBlue = withCss("--light-blue", "#458588");
const lightPurple = withCss("--light-purple", "#b16286");
const lightAqua = withCss("--light-aqua", "#689d6a");
const lightOrange = withCss("--light-orange", "#d65d0e");
const lightGray = withCss("--light-gray", "#928374");
const darkRed = withCss("--dark-red", "#9d0006");
const darkGreen = withCss("--dark-green", "#79740e");
const darkYellow = withCss("--dark-yellow", "#b57614");
const darkBlue = withCss("--dark-blue", "#076678");
const darkPurple = withCss("--dark-purple", "#8f3f71");
const darkAqua = withCss("--dark-aqua", "#427b58");
const darkOrange = withCss("--dark-orange", "#af3a03");
const darkGray = withCss("--dark-gray", "#7c6f64");
