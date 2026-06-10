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
  crepus.toml           # target + SEO config + head_html (script injection)
  runtime/              # Rust WASM crate
    src/lib.rs          # wasm_bindgen: crepus_render(bundle_json)
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

### `<style>` blocks

Raw HTML `<style>...</style>` tags at document level let you write CSS with `{ }` (which crepus would otherwise eat as expression interpolation). The docs-site uses this pattern.

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

### slot-rotate

```
slot-rotate interval={8000}
  "first phrase"
  "second phrase"
```

Cycles through phrases with a crossfade animation. Use `interval={ms}` to control timing.

### head_html (crepus.toml)

Inject raw HTML (scripts, styles, meta tags) into `<head>`:

```toml
[[targets]]
head_html = """
<script>...</script>
"""
```

Use for interactive JavaScript that needs to run after the crepus WASM render. Poll for `#crepus-root` children before accessing rendered elements.

### UnoCSS

UnoCSS runtime (`vendor/unocss.js`) extracts classes from rendered DOM. Standard preset-wind utilities work.

## Key crates

- `crepuscularity-web` — HTML rendering, bundle parser
- `crepuscularity-core` — parser, AST, context, eval