# CroftSoft Dioxus Static Hydration Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-static-hydration/blob/main/LICENSE.txt

- Prototype of static prerendering with hydration using Dioxus

## Utilities Installation

- Install the Rust command line utility "cargo"
  - cargo is installed when you install Rust
  - https://www.rust-lang.org/
- Install the Dioxus Command Line Interface (CLI) "dx"
  - cargo install dioxus-cli --locked
  - https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
- Install npm
  - npm installs utilities such as prettier
  - npm scripts run the dx and cargo commands
  - npm can be installed by installing node.js
  - https://nodejs.org/

## Hot Reload

- cd dioxus-static-hydration/
- dx serve --hot-reload
- Open your browser to http://localhost:8080/
- Make changes to the HTML in src/lib.rs or the CSS in public/stylesheet.css
- Note that the changes are updated in your browser as soon as you save

## Static Prerendering with Hydration

- cd dioxus-static-hydration/
- npm install
  - Installs the utility pretter to format the HTML
  - Installs the utility http-server to serve the HTML
- npm start
  - Deletes the distribution directory dist/ to remove files from a previous run
  - Makes the index.html page with the hydration code
  - Inserts the prerendered HTML
  - Formats the HTML using the prettier utility
  - Launches http-server to show the HTML in the browser

## TODO

- Make a separate binary for the hydration step
- Bump the version number

## History

- Initial release: 2023-09-11
