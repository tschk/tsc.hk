import { join } from "node:path";

type Handler = (request: Request) => Response | Promise<Response>;

/** Match the static host's directory URLs without removing existing entry URLs. */
export function withSiteRouting(handler: Handler, publicDir: string): Handler {
  return async (request) => {
    const url = new URL(request.url);
    const isRead = request.method === "GET" || request.method === "HEAD";
    if (!isRead) return handler(request);

    const redirects: Record<string, string> = {
      "/index.html": "/",
      "/telekinesis": "/telekinesis/",
      "/telekinesis/index.html": "/telekinesis/",
    };
    const destination = redirects[url.pathname];
    if (destination) {
      return new Response(null, {
        status: 308,
        headers: { location: destination + url.search },
      });
    }

    if (url.pathname === "/sitemap.xml") {
      return new Response(
        request.method === "HEAD"
          ? null
          : Bun.file(join(publicDir, "sitemap.xml")),
        {
          headers: { "content-type": "application/xml; charset=utf-8" },
        },
      );
    }

    const response = await handler(request);
    if (response.status !== 404) return response;
    return new Response(
      request.method === "HEAD" ? null : Bun.file(join(publicDir, "404.html")),
      {
        status: 404,
        headers: {
          "content-type": "text/html; charset=utf-8",
          "x-robots-tag": "noindex",
        },
      },
    );
  };
}
