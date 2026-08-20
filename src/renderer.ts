import { crepusRenderer } from "@tschk/crepus-moonshine";
import type { Renderer, RenderContext } from "@tschk/moonshine-framework";
import { headForRoute, headHtml } from "./head";
import { pageIr, telekinesisIr } from "./ir";

function irFor(routeId: string) {
  if (routeId === "telekinesis") return telekinesisIr;
  return pageIr;
}

function injectHead(html: string, routeId: string): string {
  const stripped = html.replace(/^(<!DOCTYPE html>\s*)+/gi, "");
  return `<!DOCTYPE html>${stripped.replace("<head>", `<head>\n  ${headHtml(headForRoute(routeId))}\n`)}`;
}

export const renderer: Renderer = {
  name: "crepus",
  async render(context: RenderContext) {
    const data = irFor(context.route.id);
    const res = await crepusRenderer.render({ ...context, data });
    const text = await res.text();
    return new Response(injectHead(text, context.route.id), {
      status: res.status,
      statusText: res.statusText,
      headers: res.headers,
    });
  },
  async prerender(context: RenderContext) {
    const data = irFor(context.route.id);
    const html = await crepusRenderer.prerender({ ...context, data });
    return injectHead(html, context.route.id);
  },
};
