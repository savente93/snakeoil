# snakeoil

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/savente93/snakeoil/branch/main/graph/badge.svg)](https://codecov.io/gh/savente93/snakeoil)
[![crates.io](https://img.shields.io/crates/v/snakeoil)](https://crates.io/crates/snakeoil)
[![Docs.rs](https://docs.rs/snakeoil/badge.svg)](https://docs.rs/snakeoil)


A Python API extractor written in Rust


(Currently mostly for shits and giggles, but who knows.)

## Dev tools
To develop snakeoil you'll want to have these tools installed:

- [`just`](https://github.com/casey/just) A command runner to run (and document) workflows we run, including installing dev and publish dependencies
- [`typos-cli`](https://github.com/crate-ci/typos) Fixing typos... not that we make any... but you know, just in case.
- [`taplo-cli`](https://github.com/tamasfe/taplo) Keeping our `.toml` files nice and clean
- [`bacon`](https://github.com/Canop/bacon) A runner that will watch your files and run checks, tests, linting etc. when they change. Very useful while developing

##  Publishing Tools
If you have to publish, or otherwise fiddle with dependencies of snakeoil you'll want these installed as well:
- [`cargo-semver`](https://github.com/obi1kenobi/cargo-semver-checks) A cargo plugin to check that we haven't accidentally broken our API when we didn't mean to.
- [`cargo-edit`](https://github.com/killercup/cargo-edit) A cargo plugin for managing dependencies, incl updating them.
- [`git-cliff`](https://github.com/orhun/git-cliff) A neat tool to generate our changelog

## Template

This repo was initially setup using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and [this template](https://github.com/savente93/rust-template)

## Roadmap:

- [x] walk file tree to find python objects
- [x] parse them to extract documentation
- [] dump documentation in similar file structure to original package
- [] do parsing and reference linking inside the docs
- [] Do actual type parsing so we can link to their docs
