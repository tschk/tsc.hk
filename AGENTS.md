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
  ir-gen/               # Rust binary: index.crepus -> src/generated/view-ir.json
  tsconfig.json         # strict, ESNext, jsx: react-jsx
  src/
    ir.ts               # re-exports the generated View IR as a CrepusIr document
    generated/
      view-ir.json      # View IR emitted by ir-gen (native crepuscularity, IR_VERSION 7)
    head.ts             # HTML head metadata (title, meta, fonts)
    renderer.ts         # crepusRenderer wrapper that injects the HTML head
    server.ts           # createBunServer + createRequestHandler + renderer
    build.ts            # prerenders dist/index.html and copies public/ for Pages
  public/               # static assets (llms.txt, llms-full.txt, agent.md, robots.txt, etc.)
  test/
    site.test.ts        # server integration tests
```

## Architecture

- `ir-gen/` is a small Rust binary depending on the published `crepuscularity-native` crate. It lowers `index.crepus` through `render_template_to_ir_with_path` and writes `src/generated/view-ir.json` (IR_VERSION 7, `style.id` preserved so `#tsc-heading` binds). Run it with `bun run build:ir`.
- `src/ir.ts` exports `pageIr` — the generated View IR typed as `CrepusIr`. No parsing happens at runtime; the `.crepus` source is lowered ahead of build by native Rust, not by the WASM parser.
- `src/head.ts` exports `headHtml()` — returns the HTML head metadata string (title, description, meta tags, Google Font link for Chivo Mono).
- `src/renderer.ts` exports `renderer` — wraps `crepusRenderer` (from `@tschk/crepus-moonshine`) and injects `headHtml()` into the rendered document. Shared by the server and the static build.
- `src/server.ts` wires `createBunServer` (from `@tschk/moonshine-deploy-bun`) with `createRequestHandler` (from `@tschk/moonshine-server`) and `renderer`. A single route `/` with mode "static" serves the IR document. Static files are served from `public/`.
- `src/build.ts` prerenders the route to `dist/index.html`, copies `public/` into `dist/`, and writes `dist/.nojekyll`. This is what the Pages workflow deploys.

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
