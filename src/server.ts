import { join } from "node:path";
import { createBunServer } from "@tschk/moonshine-deploy-bun";
import { createRequestHandler } from "@tschk/moonshine-server";
import { crepusRenderer } from "@tschk/crepus-moonshine";
import type { Renderer, RenderContext } from "@tschk/moonshine-framework";
import { pageIr } from "./ir";
import { headHtml } from "./head";

const root = import.meta.dir;
const publicDir = join(root, "..", "public");

const route = {
  id: "home",
  path: "/",
  file: "src/server.ts",
  mode: "static" as const,
  runtime: "bun" as const,
  decision: "static",
  clientEntries: [] as string[],
};

function injectHead(html: string): string {
  const stripped = html.replace(/^(<!DOCTYPE html>\s*)+/gi, "");
  return `<!DOCTYPE html>${stripped.replace("<head>", `<head>\n  ${headHtml()}\n`)}`;
}

const renderer: Renderer = {
  name: "crepus",
  async render(context: RenderContext) {
    const res = await crepusRenderer.render({ ...context, data: pageIr });
    const text = await res.text();
    return new Response(injectHead(text), {
      status: res.status,
      statusText: res.statusText,
      headers: res.headers,
    });
  },
  async prerender(context: RenderContext) {
    const html = await crepusRenderer.prerender({ ...context, data: pageIr });
    return injectHead(html);
  },
};

const fetch = createRequestHandler({
  routes: [route],
  modules: {
    [route.id]: {
      loader: () => pageIr,
    },
  },
  renderer,
  staticDir: publicDir,
});

const server = createBunServer({
  fetch,
  port: Number(process.env.PORT) || 4000,
  staticDir: publicDir,
});

console.log(`tsc.hk → ${server.url.origin}`);
