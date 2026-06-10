# tsc.hk — crepus web site

Static site built with crepuscularity-web (Rust WASM renderer + UnoCSS).

## Quick start

```bash
# Dev server (hot-reload on .crepus changes)
crepus web dev --port 4000

# Production build
crepus web build --site .
# Serve dist/ with any static file server:
python3 -m http.server 4000 -d dist
```

## Project structure

```
tsc-hk/
  index.crepus          # entry template (UnoCSS classes, indent-based)
  crepus.toml           # target + SEO config
  runtime/              # Rust WASM crate
    src/lib.rs          # all interactivity lives here (web-sys + wasm-bindgen)
  dist/                 # build output
```

## Template syntax (.crepus)

- Indentation = nesting (2 spaces)
- First word = HTML tag
- Remaining words = UnoCSS classes
- `key="value"` = attributes (href, data-*, style)
- `"quoted string"` = text content
- `{expr}` = expression interpolation

### Top-level directives

```
google-fonts "Chivo Mono"
```

Loads Google Font via `<link>` and sets `font-family` on `<body>`. Placed at top of file (before any elements).

### Group elements

Define reusable class bundles at the bottom of the file:

```
.proj
  no-underline transition-all duration-200 block group text-zinc-300 hover:text-zinc-100
.name
  text-zinc-100 group-hover:text-white transition-colors duration-200
```

Then use them as tag names:

```
a proj href="..."
  span name "crepuscularity"
  span desc " — description"
```

### UnoCSS

UnoCSS runtime (`vendor/unocss.js`) extracts classes from rendered DOM. Standard preset-wind utilities work.

## Interactivity — Rust only

All interactive behavior (animations, event handling, DOM manipulation) must be
implemented in Rust inside `runtime/src/lib.rs` using `web-sys`, `js-sys`, and
`wasm-bindgen`. **Do not add JavaScript to `head_html` or anywhere else.**

Use **document-level event delegation** (listeners on `document`, not on
individual elements) so that handlers survive DOM replacement during hot-reload
and re-renders. Use `#[wasm_bindgen(start)]` to set up listeners once on WASM
load.

Pattern:

```rust
#[wasm_bindgen(start)]
pub fn start() {
    let doc = document();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let Some(target) = find_target(&event, "#my-element") else { return };
        // handle event
    }) as Box<dyn FnMut(web_sys::Event)>);
    doc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
    closure.forget();
}
```

Use `thread_local!` statics for persistent state (timers, animation flags).

## Key crates

- `crepuscularity-web` — HTML rendering, bundle parser
- `crepuscularity-core` — parser, AST, context, eval
- `web-sys` — DOM API bindings
- `js-sys` — JS interop (Math.random, Promise, Function, etc.)
