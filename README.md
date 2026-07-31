# tsc.hk

Static site for The Software Company of Hong Kong, built with `crepuscularity-moonshine` (Crepus IR renderer) on the Moonshine framework.

## Quick Start

```bash
bun install
bun run dev
```

The development server runs on port 4000 with hot-reload via `bun --watch`.

## Build

```bash
bun run build
```

The server starts and serves the site on port 4000 (or `process.env.PORT`).

Agent-readable files are served at `/llms.txt`, `/llms-full.txt`, `/agent.md`, and `/README.md` from the `public/` directory.

## Project Structure

```text
tsc-hk/
  package.json
  tsconfig.json
  src/
    ir.ts          # page content as a CrepusIr document
    head.ts        # HTML head metadata (title, meta tags, fonts)
    server.ts      # Moonshine server (createBunServer + crepusRenderer)
  public/          # static assets (llms.txt, robots.txt, etc.)
  test/
    site.test.ts   # server integration tests
```

## Architecture

The site uses `@tschk/crepus-moonshine` to render a `CrepusIr` document (defined in `src/ir.ts`) via the `crepusRenderer`. The server is created with `createBunServer` from `@tschk/moonshine-deploy-bun` and routes are handled by `createRequestHandler` from `@tschk/moonshine-server`.

Page content is defined as Crepus IR nodes (stack, text, link, list, listItem, divider) with inline styles matching the dark zinc aesthetic.

## Quality Gates

```bash
bun run typecheck
bun test
```

## Benchmarks

Comparison of the live production site (before migration, crepuscularity-web on
GitHub Pages + Cloudflare) against the local moonshine server (`bun run start`,
port 4011). Each metric is the average of 10 sequential `curl` requests.

| Metric                          | Before (crepuscularity-web, live) | After (moonshine, local) |
|---------------------------------|-----------------------------------|--------------------------|
| Avg response time               | 84.8ms                            | 2.7ms                    |
| TTFB                            | 77.7ms                            | 1.6ms                    |
| HTML size                       | 23.5KB                            | 14.4KB                   |
| External requests               | 33                                | 26                       |
| Stack                           | Rust WASM + UnoCSS                | Bun + React + Crepus IR  |

## Moonshine

Moonshine is a ground-up, Bun-first web framework built from a hyperminimal signal kernel. Start with signals; add only the routing, rendering, server, compiler, and deployment layers your project needs.

<https://github.com/tschk/moonshine>
