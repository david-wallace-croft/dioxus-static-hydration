# CroftSoft Dioxus Static Hydration Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-static-hydration/blob/main/LICENSE.txt

- Prototype of static prerendering with client-side hydration using Dioxus

## Usage

- Install the Rust command line utility "cargo"
  - cargo is installed when you install Rust
  - https://www.rust-lang.org/
- Install the Dioxus Command Line Interface (CLI) "dx"
  - cargo install dioxus-cli --locked
  - https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
- Install npm
  - npm installs utilities such as prettier
  - npm scripts runs the dx and cargo commands
  - npm can be installed by installing node.js
  - https://nodejs.org/
- npm install
  - Installs the utility pretter to format the HTML
  - Installs the utility http-server to serve the HTML
- npm start
  - Makes the index.html page with the hydration code
  - Inserts the prerendered HTML
  - Formats the HTML using the prettier utility
  - Launches http-server to show the HTML in the browser

## TODO

- Make separate binaries for the hydration and prerender steps
- Bump the version number

## History

- Initial release: 2023-09-11
