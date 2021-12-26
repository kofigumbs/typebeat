const { execSync } = require('child_process');
const { readFileSync, writeFileSync } = require('fs');
const { request } = require('https');

// https://github.com/rust-lang/cargo/blob/master/LICENSE-THIRD-PARTY
const PREFACE = `
The Typebeat source code itself does not bundle any third party libraries, but
it depends on a number of libraries which carry their own copyright notices and
license terms. These libraries are normally all linked static into the binary
distributions of Typebeat:
`;

const CARGO_OVERRIDES = {
  'atk-sys MIT': 'gtk-rs/gtk3-rs/master/LICENSE',
  'blake2b_simd MIT': 'oconnor663/blake2_simd/master/LICENSE',
  'block MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'cocoa-foundation MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'cocoa-foundation MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'convert_case MIT': 'rutrum/convert-case/master/LICENSE',
  'core-graphics-types MIT': 'servo/core-foundation-rs/master/LICENSE-MIT',
  'dispatch MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'enumflags2_derive MIT': 'NieDzejkob/enumflags2/master/LICENSE-MIT',
  'ep-miniaudio-sys MIT': 'ExPixel/miniaudio-rs/master/LICENSE',
  'field-offset MIT': 'Diggsey/rust-field-offset/master/LICENSE-MIT',
  'gdk-pixbuf-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'gdk-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'gio-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'glib-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'gobject-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'gtk-sys MIT': 'gtk-rs/gtk3-rs/master/LICENSE',
  'kuchiki MIT': 'kuchiki-rs/kuchiki/master/LICENSE',
  'malloc_buf MIT': 'SSheldon/malloc_buf/master/LICENSE',
  'miniaudio MIT': 'ExPixel/miniaudio-rs/master/LICENSE',
  'ndk MIT': 'rust-windowing/android-ndk-rs/master/LICENSE-MIT',
  'ndk-glue MIT': 'rust-windowing/android-ndk-rs/master/LICENSE-MIT',
  'ndk-macro MIT': 'rust-windowing/android-ndk-rs/master/LICENSE-MIT',
  'ndk-sys MIT': 'rust-windowing/android-ndk-rs/master/LICENSE-MIT',
  'objc-foundation MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'objc_id MIT': 'SSheldon/rust-objc/master/LICENSE.txt',
  'pango-sys MIT': 'gtk-rs/gtk-rs-core/master/LICENSE',
  'phf MIT': 'rust-phf/rust-phf/master/LICENSE',
  'phf_generator MIT': 'rust-phf/rust-phf/master/LICENSE',
  'phf_macros MIT': 'rust-phf/rust-phf/master/LICENSE',
  'phf_shared MIT': 'rust-phf/rust-phf/master/LICENSE',
  'remove_dir_all MIT': 'XAMPPRocky/remove_dir_all/master/LICENCE-MIT',
  'security-framework MIT': 'kornelski/rust-security-framework/master/LICENSE-MIT',
  'security-framework-sys MIT': 'kornelski/rust-security-framework/master/LICENSE-MIT',
  'selectors MPL-2.0': 'servo/servo/master/LICENSE',
  'void MIT': 'reem/rust-void/master/LICENSE-MIT',
  'webview2-com MIT': 'wravery/webview2-rs/main/LICENSE',
  'webview2-com-macros MIT': 'wravery/webview2-rs/main/LICENSE',
  'webview2-com-sys MIT': 'wravery/webview2-rs/main/LICENSE',
  'winapi-i686-pc-windows-gnu MIT': 'retep998/winapi-rs/0.3/LICENSE-MIT',
  'winapi-x86_64-pc-windows-gnu MIT': 'retep998/winapi-rs/0.3/LICENSE-MIT',
  'windows MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_aarch64_msvc MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_gen MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_i686_gnu MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_i686_msvc MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_macros MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_quote MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_reader MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_x86_64_gnu MIT': 'microsoft/windows-rs/master/.github/license-mit',
  'windows_x86_64_msvc MIT': 'microsoft/windows-rs/master/.github/license-mit',
};

const cache = {};
const load = (url, fallback, resolve, response) => {
  let body = '';
  response.on('data', x => body += x);
  response.on('end', () => resolve(cache[url] = (response.statusCode === 200 ? body : fallback)));
};
const get = (url, fallback) => new Promise(resolve => {
  cache[url]
    ? resolve(cache[url])
    : request(url, response => load(url, fallback, resolve, response)).end();
});

const attribution = (title, license) => `
${title}
======
${license}
`;

const cargoAttribution = async crate => {
  const { license, text } = crate.licenses[0];
  const override = CARGO_OVERRIDES[`${crate.package_name} ${license}`];
  const fallback = `<https://opensource.org/licenses/${license}>`;
  return attribution(
    crate.package_name,
    text !== 'NOT FOUND' ? text : await get(`https://raw.githubusercontent.com/${override}`, fallback),
  );
};

(async () => {
  const crates = JSON.parse(execSync('cargo bundle-licenses --format json', { maxBuffer: 4000000 }));
  const attributions = await Promise.all(crates.third_party_libraries.map(cargoAttribution));
  attributions.push(attribution('firacode', readFileSync('node_modules/firacode/LICENSE')));
  attributions.push(attribution('solid-js', readFileSync('node_modules/solid-js/LICENSE')));
  writeFileSync('LICENSE-THIRD-PARTY', PREFACE.concat(attributions.join('')).trim());
})();
