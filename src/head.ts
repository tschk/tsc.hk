export type HeadMeta = {
  title: string;
  description: string;
  canonical: string;
};

export const homeHead: HeadMeta = {
  title: "The Software Company of Hong Kong — tsc.hk",
  description:
    "an independent R&D lab building systems software, runtimes, developer tools, application platforms, operating systems, browsers, and programming languages.",
  canonical: "https://tsc.hk/",
};

export const telekinesisHead: HeadMeta = {
  title: "telekinesis — tsc.hk",
  description:
    "AI coding agent CLI + TUI. Powered by the rotary (rx4) harness engine and crepuscularity-tui.",
  canonical: "https://tsc.hk/telekinesis/",
};

export function headForRoute(routeId: string): HeadMeta {
  if (routeId === "telekinesis") return telekinesisHead;
  return homeHead;
}

function escapeHtml(value: string): string {
  return value.replace(
    /[&<>"']/g,
    (character) =>
      ({
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#39;",
      })[character]!,
  );
}

function structuredData(meta: HeadMeta): string {
  const organization = "https://tsc.hk/#organization";
  const website = "https://tsc.hk/#website";
  return JSON.stringify({
    "@context": "https://schema.org",
    "@graph": [
      {
        "@type": "Organization",
        "@id": organization,
        name: "The Software Company of Hong Kong",
        url: homeHead.canonical,
        sameAs: [
          "https://github.com/tschk",
          "https://x.com/tsc_hk",
          "https://instagram.com/thesoftwarecompanyofhongkong",
        ],
      },
      {
        "@type": "WebSite",
        "@id": website,
        name: "The Software Company of Hong Kong",
        url: homeHead.canonical,
        publisher: { "@id": organization },
        inLanguage: "en",
      },
      {
        "@type": "WebPage",
        "@id": `${meta.canonical}#webpage`,
        url: meta.canonical,
        name: meta.title,
        description: meta.description,
        isPartOf: { "@id": website },
        inLanguage: "en",
      },
    ],
  }).replace(/</g, "\\u003c");
}

export function headHtml(meta: HeadMeta = homeHead): string {
  const title = escapeHtml(meta.title);
  const description = escapeHtml(meta.description);
  const canonical = escapeHtml(meta.canonical);
  return [
    '<meta charset="utf-8" />',
    '<meta name="viewport" content="width=device-width, initial-scale=1" />',
    `<title>${title}</title>`,
    `<meta name="description" content="${description}" />`,
    '<meta name="robots" content="index,follow" />',
    `<link rel="canonical" href="${canonical}" />`,
    '<meta property="og:type" content="website" />',
    '<meta property="og:site_name" content="The Software Company of Hong Kong" />',
    `<meta property="og:title" content="${title}" />`,
    `<meta property="og:description" content="${description}" />`,
    `<meta property="og:url" content="${canonical}" />`,
    '<meta name="twitter:card" content="summary" />',
    '<meta name="twitter:site" content="@tsc_hk" />',
    '<meta name="twitter:creator" content="@tsc_hk" />',
    `<meta name="twitter:title" content="${title}" />`,
    `<meta name="twitter:description" content="${description}" />`,
    `<script type="application/ld+json">${structuredData(meta)}</script>`,
    '<link rel="preconnect" href="https://fonts.googleapis.com" />',
    '<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />',
    '<link href="https://fonts.googleapis.com/css2?family=Chivo+Mono&display=swap" rel="stylesheet" />',
    '<link rel="stylesheet" href="/site.css" />',
    '<script type="module">import init from "/pkg/tsc_hk_runtime.js";init("/pkg/tsc_hk_runtime_bg.wasm");</script>',
  ].join("\n  ");
}
