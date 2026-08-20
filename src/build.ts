import { cp, mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import type { RenderContext, RouteArtifact } from "@tschk/moonshine-framework";
import { homeRoute, telekinesisRoute } from "./app";
import { renderer } from "./renderer";

async function prerender(
  route: RouteArtifact,
  requestUrl: string,
  outFile: string,
): Promise<void> {
  const ctx: RenderContext = {
    request: new Request(requestUrl),
    route,
    params: {},
    data: {},
    signal: new AbortController().signal,
  };
  const html = await renderer.prerender(ctx);
  await mkdir(join(outFile, ".."), { recursive: true });
  await writeFile(outFile, html);
}

async function main(): Promise<void> {
  const root = join(import.meta.dir, "..");
  const outDir = join(root, "dist");
  await mkdir(outDir, { recursive: true });
  await cp(join(root, "public"), outDir, { recursive: true });
  await cp(join(root, "runtime", "pkg"), join(outDir, "pkg"), {
    recursive: true,
  });
  await prerender(homeRoute, "https://tsc.hk/", join(outDir, "index.html"));
  await prerender(
    telekinesisRoute,
    "https://tsc.hk/telekinesis",
    join(outDir, "telekinesis", "index.html"),
  );
  await writeFile(join(outDir, ".nojekyll"), "");
  console.log(`wrote ${outDir}`);
}

main();
