import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { createBunServer } from "@tschk/moonshine-deploy-bun";
import { createRequestHandler } from "@tschk/moonshine-server";
import { crepusRenderer } from "@tschk/crepus-moonshine";
import type { Renderer, RenderContext } from "@tschk/moonshine-framework";
import { pageIr } from "../src/ir";
import { headHtml } from "../src/head";

const publicDir = join(import.meta.dir, "../public");

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

describe("tsc.hk site", () => {
  test("GET / returns 200 with HTML content", async () => {
    const server = createBunServer({ fetch, port: 0, staticDir: publicDir });
    try {
      const res = await fetch(new Request(`${server.url.origin}/`));
      expect(res.status).toBe(200);
      expect(res.headers.get("content-type")).toContain("text/html");
      const html = await res.text();
      expect(html).toContain("the software company of hong kong");
      expect(html).toContain("PROJECTS");
      expect(html).toContain("crepuscularity");
      expect(html).toContain("CONTACT");
      expect(html).toContain("site built with crepuscularity + moonshine");
      expect(html).toContain('class="min-h-screen');
      expect(html).not.toContain("style=");
      expect(html).toContain("The Software Company of Hong Kong — tsc.hk");
    } finally {
      await server.stop(true);
    }
  });

  test("GET /llms.txt returns 200", async () => {
    const server = createBunServer({ fetch, port: 0, staticDir: publicDir });
    try {
      const res = await fetch(new Request(`${server.url.origin}/llms.txt`));
      expect(res.status).toBe(200);
      const text = await res.text();
      expect(text).toContain("The Software Company of Hong Kong");
    } finally {
      await server.stop(true);
    }
  });
});
