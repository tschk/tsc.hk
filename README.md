# tsc.hk

Static site for The Software Company of Hong Kong, built with `crepuscularity-web`, a Rust WASM runtime, and UnoCSS.

## Quick Start

```bash
crepus web dev --port 4000
```

The development server watches `.crepus` files and rebuilds the Rust WASM runtime for local preview.

## Build

```bash
crepus web build --site .
```

The production site is emitted to `dist/`.

To preview the built output:

```bash
python3 -m http.server 4000 -d dist
```

## Project Structure

```text
tsc-hk/
  index.crepus
  crepus.toml
  runtime/
    Cargo.toml
    src/lib.rs
  dist/
```

## Template Files

`index.crepus` is an indentation-based template:

- two spaces define nesting
- the first word is the HTML tag
- remaining words are UnoCSS classes
- quoted strings become text nodes
- attributes use `key="value"`
- group elements at the bottom of the file define reusable class bundles

The template loads `Chivo Mono` with the `google-fonts` directive and relies on the UnoCSS runtime in the generated site.

## Runtime

All interactivity belongs in `runtime/src/lib.rs`.

The runtime is Rust compiled to WASM with `wasm-bindgen`, `web-sys`, and `js-sys`. Event handlers should use document-level delegation so they survive hot reload and DOM replacement.

Do not add JavaScript to `head_html` or directly to the template.

## Quality Gates

```bash
cargo fmt --manifest-path runtime/Cargo.toml --check
cargo check --manifest-path runtime/Cargo.toml
crepus web build --site .
```

There is currently no separate automated test suite for this site.
