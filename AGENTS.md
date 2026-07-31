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
    server.ts           # createBunServer + createRequestHandler + crepusRenderer
  public/               # static assets (llms.txt, llms-full.txt, agent.md, robots.txt, etc.)
  test/
    site.test.ts        # server integration tests
```

## Architecture

- `src/ir.ts` exports `pageIr` — a `CrepusIr` document using node kinds: stack, text, link, list, listItem, divider. Inline styles match the dark zinc aesthetic (bg #09090b, text zinc-300, monospace font).
- `src/head.ts` exports `headHtml()` — returns the HTML head metadata string (title, description, meta tags, Google Font link for Chivo Mono).
- `src/server.ts` wires `createBunServer` (from `@tschk/moonshine-deploy-bun`) with `createRequestHandler` (from `@tschk/moonshine-server`) and `crepusRenderer` (from `@tschk/crepus-moonshine`). A single route `/` with mode "static" serves the IR document. Static files are served from `public/`.

## Dependencies

Moonshine packages are referenced via `file:../moonshine/packages/*` since tsc.hk is not a workspace monorepo. The `overrides` field ensures all transitive `@tschk/moonshine-*` dependencies resolve locally.

## Quality gates

```bash
bun run typecheck
bun test
```

## Moonshine

Moonshine is a ground-up, Bun-first web framework built from a hyperminimal signal kernel. Start with signals; add only the routing, rendering, server, compiler, and deployment layers your project needs.

<https://github.com/tschk/moonshine>
