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
  tsconfig.json         # strict, ESNext, jsx: react-jsx
  src/
    ir.ts               # page content as a CrepusIr document
    head.ts             # HTML head metadata (title, meta, fonts)
    renderer.ts         # crepusRenderer wrapper that injects the HTML head
    server.ts           # createBunServer + createRequestHandler + renderer
    build.ts            # prerenders dist/index.html and copies public/ for Pages
  public/               # static assets (llms.txt, llms-full.txt, agent.md, robots.txt, etc.)
  test/
    site.test.ts        # server integration tests
```

## Architecture

- `src/ir.ts` exports `pageIr` — a `CrepusIr` document using node kinds: stack, text, link, list, listItem, divider. Inline styles match the dark zinc aesthetic (bg #09090b, text zinc-300, monospace font).
- `src/head.ts` exports `headHtml()` — returns the HTML head metadata string (title, description, meta tags, Google Font link for Chivo Mono).
- `src/renderer.ts` exports `renderer` — wraps `crepusRenderer` (from `@tschk/crepus-moonshine`) and injects `headHtml()` into the rendered document. Shared by the server and the static build.
- `src/server.ts` wires `createBunServer` (from `@tschk/moonshine-deploy-bun`) with `createRequestHandler` (from `@tschk/moonshine-server`) and `renderer`. A single route `/` with mode "static" serves the IR document. Static files are served from `public/`.
- `src/build.ts` prerenders the route to `dist/index.html`, copies `public/` into `dist/`, and writes `dist/.nojekyll`. This is what the Pages workflow deploys.

## Dependencies

Moonshine packages are consumed from npm under the `@tschk` scope (`^0.3.1`), with `@tschk/crepuscularity-wasm` at `^0.1.0`. No `overrides` are needed: every published package already pins its `@tschk` dependencies to the same ranges.

## Quality gates

```bash
bun run typecheck
bun test
bun run build
```

## Moonshine

Moonshine is a ground-up, Bun-first web framework built from a hyperminimal signal kernel. Start with signals; add only the routing, rendering, server, compiler, and deployment layers your project needs.

<https://github.com/tschk/moonshine>
