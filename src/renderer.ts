import { crepusRenderer } from "@tschk/crepus-moonshine";
import type { Renderer, RenderContext } from "@tschk/moonshine-framework";
import { pageIr } from "./ir";
import { headHtml } from "./head";

function injectHead(html: string): string {
  const stripped = html.replace(/^(<!DOCTYPE html>\s*)+/gi, "");
  return `<!DOCTYPE html>${stripped.replace("<head>", `<head>\n  ${headHtml()}\n`)}`;
}

export const renderer: Renderer = {
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
