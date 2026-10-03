import { headForRoute, headHtml } from "./head";

/** IR v7 retains classes and IDs, but its stack/text nodes lose HTML semantics. */
export async function siteDocument(
  html: string,
  routeId: string,
): Promise<string> {
  const stripped = html.replace(/^(<!DOCTYPE html>\s*)+/gi, "");
  return (
    new HTMLRewriter()
      .on("html", {
        element: (element) => {
          element.setAttribute("lang", "en");
        },
      })
      .on("head", {
        element: (element) => {
          element.prepend(`\n  ${headHtml(headForRoute(routeId))}\n`, {
            html: true,
          });
        },
      })
      .on("[data-crepus-root]", {
        element: (element) => {
          element.tagName = "main";
        },
      })
      .on("#tsc-heading, #tk-heading", {
        element: (element) => {
          element.tagName = "h1";
        },
      })
      // This class bundle identifies the section titles in both .crepus templates.
      .on(".text-zinc-100.font-medium.mt-4", {
        element: (element) => {
          element.tagName = "h2";
        },
      })
      .transform(new Response(`<!DOCTYPE html>${stripped}`))
      .text()
  );
}
