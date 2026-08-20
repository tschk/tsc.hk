# tsc.hk — moonshine site

Static site built with crepuscularity-moonshine (Crepus IR renderer) on the Moonshine framework, served by Bun.

## Quick start

```bash
bun install

# Dev server (hot-reload via bun --watch)
bun run dev

# Production server
bun run start
```

The server listens on `process.env.PORT` or 4000.

## Project structure

```
tsc-hk/
  package.json          # deps + scripts
  ir-gen/               # Rust binary: *.crepus -> src/generated/*-ir.json
  tsconfig.json         # strict, ESNext, jsx: react-jsx
  index.crepus          # homepage
  telekinesis.crepus    # /telekinesis product page
  src/
    ir.ts               # re-exports generated View IR documents as CrepusIr
    generated/
      view-ir.json      # homepage View IR (native crepuscularity, IR_VERSION 7)
      telekinesis-ir.json
    head.ts             # HTML head metadata (title, meta, fonts)
    renderer.ts         # crepusRenderer wrapper that injects the HTML head
    app.ts              # routes + createRequestHandler
    server.ts           # createBunServer + app fetch
    build.ts            # prerenders dist/index.html and dist/telekinesis/index.html
  public/               # static assets (llms.txt, llms-full.txt, agent.md, robots.txt, etc.)
  test/
    site.test.ts        # server integration tests
```

## Architecture

- `ir-gen/` is a small Rust binary depending on the published `crepuscularity-native` crate. It lowers `.crepus` sources through `render_template_to_ir_with_path` and writes `src/generated/view-ir.json` and `src/generated/telekinesis-ir.json` (IR_VERSION 7, `style.id` preserved so `#tsc-heading` and `#tk-heading` bind). Run it with `bun run build:ir`.
- `src/ir.ts` exports `pageIr` and `telekinesisIr` — generated View IR typed as `CrepusIr`. No parsing happens at runtime; the `.crepus` source is lowered ahead of build by native Rust, not by the WASM parser.
- `src/head.ts` exports `headHtml()` — returns the HTML head metadata string (title, description, meta tags, Google Font link for Chivo Mono), selected per route.
- `src/renderer.ts` exports `renderer` — wraps `crepusRenderer` (from `@tschk/crepus-moonshine`) and injects `headHtml()` into the rendered document. Shared by the server and the static build.
- `src/app.ts` wires `createRequestHandler` (from `@tschk/moonshine-server`) with static routes `/` and `/telekinesis`. `src/server.ts` wraps that handler with `createBunServer` (from `@tschk/moonshine-deploy-bun`). Static files are served from `public/`.
- `src/build.ts` prerenders `/` to `dist/index.html` and `/telekinesis` to `dist/telekinesis/index.html`, copies `public/` into `dist/`, and writes `dist/.nojekyll`. This is what the Pages workflow deploys. This repo does not attach `telekinesis.tsc.hk`; that needs a Cloudflare DNS CNAME plus a redirect or rewrite to `https://tsc.hk/telekinesis`.

## Dependencies

Moonshine packages are consumed from npm under the `@tschk` scope (`^0.3.1`). No `overrides` are needed: every published package already pins its `@tschk` dependencies to the same ranges. The `.crepus` parser is the native `crepuscularity-native` crate from crates.io, consumed through `ir-gen/`, not the WASM npm parser.

## Quality gates

```bash
bun run build:ir
bun run typecheck
bun test
bun run build
```

## Moonshine

Moonshine is a ground-up, Bun-first web framework built from a hyperminimal signal kernel. Start with signals; add only the routing, rendering, server, compiler, and deployment layers your project needs.

<https://github.com/tschk/moonshine>
