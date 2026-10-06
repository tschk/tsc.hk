import { crepusRenderer } from "@tschk/crepus-moonshine";
import type { Renderer, RenderContext } from "@tschk/moonshine-framework";
import { siteDocument } from "./document";
import { pageIr, telekinesisIr } from "./ir";

function irFor(routeId: string) {
  if (routeId === "telekinesis") return telekinesisIr;
  return pageIr;
}

export const renderer: Renderer = {
  name: "crepus",
  async render(context: RenderContext) {
    const data = irFor(context.route.id);
    const res = await crepusRenderer.render({ ...context, data });
    const text = await res.text();
    return new Response(await siteDocument(text, context.route.id), {
      status: res.status,
      statusText: res.statusText,
      headers: res.headers,
    });
  },
  async prerender(context: RenderContext) {
    const data = irFor(context.route.id);
    const html = await crepusRenderer.prerender({ ...context, data });
    return siteDocument(html, context.route.id);
  },
};
