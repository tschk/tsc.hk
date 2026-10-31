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

The build prerenders the homepage and `/telekinesis/` into `dist/`, alongside
the static assets. Run `bun run start` to serve the source site on port 4000
(or `process.env.PORT`).

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

Page content comes from `index.crepus` and `telekinesis.crepus`, lowered into
`src/generated/` by `bun run build:ir`. The native Crepus compiler preserves the
utility classes, and `public/site.css` supplies their styles without a browser
JavaScript compiler. Update that stylesheet when adding utility classes; the
SEO tests check coverage of the generated IR's class tokens.

`src/document.ts` adds document language, main landmarks and headings after
Moonshine renders the IR (whose stack/text nodes do not retain HTML tag names).
It preserves the heading IDs and classes used by the optional Rust animations.
The complete page remains visible and styled without JavaScript or WebAssembly.

The sitemap lists the final canonical URLs `/` and `/telekinesis/`. The Bun
handler redirects the old directory entry URLs and returns a real 404 for
missing pages. `public/404.html` is also copied into the static build, preventing
Cloudflare Pages from treating unknown paths as homepage SPA routes.

## Deployment

Pushes to `master` build and test the site, then upload the same `dist/`
artifact to the **tschk-hk** Cloudflare Pages project (`tschk-hk.pages.dev`),
which serves `tsc.hk`. GitHub Pages is not the production target.

The `cloudflare-pages` GitHub environment needs a `CLOUDFLARE_API_TOKEN`
secret with Pages Edit access to account `fc62a6e6528bec6d3d81c3bf8967ceeb`.
A missing credential fails deployment rather than skipping it. The workflow
checks `https://tsc.hk/deployed-revision.txt` against the merged commit and
smoke-tests the public homepage and `/telekinesis/` before reporting success.
The Actions run summary records the verified revision. Cloudflare Pages keeps
previous production deployments available for rollback.

## Quality Gates

```bash
bun run build:ir
bun run typecheck
bun test
bun run build:runtime
bun run build
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
