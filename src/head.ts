export type HeadMeta = {
  title: string;
  description: string;
  canonical: string;
};

export const homeHead: HeadMeta = {
  title: "The Software Company of Hong Kong — tsc.hk",
  description:
    "an independent R&D lab building systems software, runtimes, developer tools, application platforms, operating systems, browsers, and programming languages.",
  canonical: "https://tsc.hk",
};

export const telekinesisHead: HeadMeta = {
  title: "telekinesis — tsc.hk",
  description:
    "AI coding agent CLI + TUI. Powered by the rotary (rx4) harness engine and crepuscularity-tui.",
  canonical: "https://tsc.hk/telekinesis",
};

export function headForRoute(routeId: string): HeadMeta {
  if (routeId === "telekinesis") return telekinesisHead;
  return homeHead;
}

export function headHtml(meta: HeadMeta = homeHead): string {
  return [
    '<meta charset="utf-8" />',
    '<meta name="viewport" content="width=device-width, initial-scale=1" />',
    `<title>${meta.title}</title>`,
    `<meta name="description" content="${meta.description}" />`,
    '<meta name="robots" content="index,follow" />',
    `<link rel="canonical" href="${meta.canonical}" />`,
    '<meta name="twitter:card" content="summary" />',
    '<meta name="twitter:site" content="@tsc_hk" />',
    '<meta name="twitter:creator" content="@tsc_hk" />',
    '<link rel="preconnect" href="https://fonts.googleapis.com" />',
    '<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />',
    '<link href="https://fonts.googleapis.com/css2?family=Chivo+Mono&display=swap" rel="stylesheet" />',
    "<style>body{font-family:'Chivo Mono',ui-monospace,monospace;margin:0}[data-crepus-root]{opacity:0}[data-crepus-root].ready{opacity:1}</style>",
    '<script src="/vendor/unocss.js"></script>',
    '<script>window.addEventListener("load",function(){document.querySelectorAll("[data-crepus-root]").forEach(function(n){n.classList.add("ready")})})</script>',
    '<script type="module">import init from "/pkg/tsc_hk_runtime.js";init("/pkg/tsc_hk_runtime_bg.wasm");</script>',
  ].join("\n  ");
}
