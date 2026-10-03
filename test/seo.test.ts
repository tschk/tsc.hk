import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { siteDocument } from "../src/document";
import { headHtml, homeHead, telekinesisHead } from "../src/head";
import { withSiteRouting } from "../src/http";

const publicDir = join(import.meta.dir, "..", "public");

describe("server-rendered document semantics", () => {
  test("static CSS covers every utility used by the generated pages", async () => {
    const css = await Bun.file(join(publicDir, "site.css")).text();
    const classes = new Set<string>();
    function visit(value: unknown): void {
      if (Array.isArray(value)) {
        value.forEach(visit);
      } else if (value && typeof value === "object") {
        const node = value as { style?: { classes?: string[] } };
        node.style?.classes?.forEach((token) => classes.add(token));
        Object.values(value).forEach(visit);
      }
    }
    for (const file of ["view-ir.json", "telekinesis-ir.json"]) {
      visit(
        await Bun.file(join(import.meta.dir, "../src/generated", file)).json(),
      );
    }
    expect(classes.size).toBeGreaterThan(0);
    for (const token of classes) {
      expect(css).toContain(`.${token.replaceAll(":", "\\:")}`);
    }
  });

  test("preserves content, classes and runtime hooks while adding native landmarks", async () => {
    const html = await siteDocument(
      '<!DOCTYPE html><!DOCTYPE html><html><head></head><body><div data-crepus-root="true"><div id="tsc-heading" class="text-lg"><span>the software company of hong kong</span></div><div class="text-zinc-100 font-medium mt-4"><span>PROJECTS</span></div><a href="/telekinesis/">telekinesis guide</a></div></body></html>',
      "home",
    );
    expect(html.match(/<!DOCTYPE html>/gi)).toHaveLength(1);
    expect(html).toContain('<html lang="en">');
    expect(html).toContain('<main data-crepus-root="true">');
    expect(html).toContain(
      '<h1 id="tsc-heading" class="text-lg"><span>the software company of hong kong</span></h1>',
    );
    expect(html).toContain(
      '<h2 class="text-zinc-100 font-medium mt-4"><span>PROJECTS</span></h2>',
    );
    expect(html).toContain('<a href="/telekinesis/">telekinesis guide</a>');
    expect(html).toContain('href="/site.css"');
    expect(html).not.toContain("opacity:0");
    expect(html).not.toContain("unocss.js");
    expect(html).not.toContain('addEventListener("load"');
    expect(html).toContain("/pkg/tsc_hk_runtime_bg.wasm");
  });

  test("uses the product page's title, canonical and heading", async () => {
    const html = await siteDocument(
      '<html><head></head><body><div data-crepus-root="true"><div id="tk-heading">telekinesis</div></div></body></html>',
      "telekinesis",
    );
    expect(html).toContain('<h1 id="tk-heading">telekinesis</h1>');
    expect(html).toContain("<title>telekinesis — tsc.hk</title>");
    expect(html).toContain(
      'rel="canonical" href="https://tsc.hk/telekinesis/"',
    );
  });
});

describe("metadata", () => {
  test.each([homeHead, telekinesisHead])(
    "uses consistent social metadata and factual structured data for $canonical",
    (meta) => {
      const html = headHtml(meta);
      expect(html).toContain(`property="og:url" content="${meta.canonical}"`);
      expect(html).toContain('name="twitter:title"');
      const json = html.match(
        /<script type="application\/ld\+json">(.*?)<\/script>/s,
      )![1];
      const graph = JSON.parse(json)["@graph"];
      expect(graph.map((node: { "@type": string }) => node["@type"])).toEqual([
        "Organization",
        "WebSite",
        "WebPage",
      ]);
      expect(graph[2].url).toBe(meta.canonical);
      expect(graph[2].name).toBe(meta.title);
      expect(graph[2].description).toBe(meta.description);
      expect(json).not.toContain("aggregateRating");
      expect(json).not.toContain("postalAddress");
    },
  );

  test("escapes attribute/text metadata and cannot break out of the JSON-LD script", () => {
    const value = '<tag> & "quote" </script><script>alert(1)</script>';
    const html = headHtml({ ...homeHead, title: value, description: value });
    expect(html).toContain("&lt;tag&gt; &amp; &quot;quote&quot;");
    expect(html).not.toContain(value);
    const json = html.match(
      /<script type="application\/ld\+json">(.*?)<\/script>/s,
    )![1];
    expect(JSON.parse(json)["@graph"][2].name).toBe(value);
  });
});

describe("crawlable URLs and errors", () => {
  const handler = withSiteRouting(
    () => new Response("missing", { status: 404 }),
    publicDir,
  );

  test.each([
    ["/index.html?ref=bookmark", "/?ref=bookmark"],
    ["/telekinesis?ref=bookmark", "/telekinesis/?ref=bookmark"],
    ["/telekinesis/index.html", "/telekinesis/"],
  ])(
    "redirects the legacy entry %s without losing query parameters",
    async (path, target) => {
      const response = await handler(new Request(`https://tsc.hk${path}`));
      expect(response.status).toBe(308);
      expect(response.headers.get("location")).toBe(target);
      expect(await response.text()).toBe("");
    },
  );

  test("returns a real noindex 404 without reflecting the requested path or homepage metadata", async () => {
    const response = await handler(
      new Request("https://tsc.hk/not-a-real-page?value=untrusted"),
    );
    expect(response.status).toBe(404);
    expect(response.headers.get("x-robots-tag")).toBe("noindex");
    const html = await response.text();
    expect(html).toContain("Page not found");
    expect(html).toContain('href="/"');
    expect(html).not.toContain("untrusted");
    expect(html).not.toContain('rel="canonical"');
  });

  test("serves XML sitemap with only final canonical URLs and advertises it in robots.txt", async () => {
    const response = await handler(new Request("https://tsc.hk/sitemap.xml"));
    expect(response.status).toBe(200);
    expect(response.headers.get("content-type")).toBe(
      "application/xml; charset=utf-8",
    );
    const xml = await response.text();
    expect(
      [...xml.matchAll(/<loc>(.*?)<\/loc>/g)].map((match) => match[1]),
    ).toEqual([homeHead.canonical, telekinesisHead.canonical]);
    expect(xml).not.toContain("<lastmod>");
    const robots = await Bun.file(join(publicDir, "robots.txt")).text();
    expect(robots).toContain("Allow: /");
    expect(robots).toContain("Sitemap: https://tsc.hk/sitemap.xml");
  });

  test.each(["/missing", "/sitemap.xml"])(
    "HEAD %s keeps status and sends no body",
    async (path) => {
      const get = await handler(new Request(`https://tsc.hk${path}`));
      const head = await handler(
        new Request(`https://tsc.hk${path}`, { method: "HEAD" }),
      );
      expect(head.status).toBe(get.status);
      expect(head.headers.get("content-type")).toBe(
        get.headers.get("content-type"),
      );
      expect(await head.text()).toBe("");
    },
  );

  test("leaves success, failure and non-read method handling to the framework", async () => {
    const success = new Response("rendered page", { status: 200 });
    expect(
      await withSiteRouting(
        () => success,
        publicDir,
      )(new Request("https://tsc.hk/")),
    ).toBe(success);
    const failure = new Response("unavailable", { status: 503 });
    expect(
      await withSiteRouting(
        () => failure,
        publicDir,
      )(new Request("https://tsc.hk/")),
    ).toBe(failure);
    const method = new Response(null, { status: 405 });
    expect(
      await withSiteRouting(
        () => method,
        publicDir,
      )(new Request("https://tsc.hk/telekinesis", { method: "POST" })),
    ).toBe(method);
  });
});
