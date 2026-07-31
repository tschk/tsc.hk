import { cp, mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import type { RenderContext, RouteArtifact } from "@tschk/moonshine-framework";
import { pageIr } from "./ir";
import { renderer } from "./renderer";

const route: RouteArtifact = {
  id: "home",
  path: "/",
  file: "src/server.ts",
  mode: "static",
  runtime: "bun",
  decision: "static",
  clientEntries: [],
};

async function main(): Promise<void> {
  const root = join(import.meta.dir, "..");
  const outDir = join(root, "dist");
  const ctx: RenderContext = {
    request: new Request("https://tsc.hk/"),
    route,
    params: {},
    data: pageIr,
    signal: new AbortController().signal,
  };
  const html = await renderer.prerender(ctx);
  await mkdir(outDir, { recursive: true });
  await cp(join(root, "public"), outDir, { recursive: true });
  await writeFile(join(outDir, "index.html"), html);
  await writeFile(join(outDir, ".nojekyll"), "");
  console.log(`wrote ${outDir}`);
}

main();
