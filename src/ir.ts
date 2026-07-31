import type { CrepusIr, CrepusNode, StyleMap } from "@tschk/crepus-moonshine";

const zinc = {
  950: "#09090b",
  900: "#18181b",
  800: "#27272a",
  700: "#3f3f46",
  600: "#52525b",
  500: "#71717a",
  400: "#a1a1aa",
  300: "#d4d4d8",
  200: "#e4e4e7",
  100: "#f4f4f5",
} as const;

const pageStyle: StyleMap = {
  minHeight: "100vh",
  backgroundColor: zinc[950],
  color: zinc[300],
  padding: "24px",
  fontSize: "14px",
  lineHeight: "1.625",
  fontFamily: '"Chivo Mono", ui-monospace, monospace',
};

const containerStyle: StyleMap = {
  maxWidth: "672px",
  display: "flex",
  flexDirection: "column",
  gap: "32px",
};

const headingStyle: StyleMap = {
  color: zinc[100],
  fontSize: "18px",
  fontWeight: 600,
  letterSpacing: "0.025em",
};

const descriptionStyle: StyleMap = {
  color: zinc[400],
  lineHeight: "1.5",
};

const sectionLabelStyle: StyleMap = {
  color: zinc[100],
  fontWeight: 500,
  marginTop: "16px",
};

const projectsStyle: StyleMap = {
  display: "flex",
  flexDirection: "column",
  gap: "12px",
  marginTop: "8px",
};

const contactsStyle: StyleMap = {
  display: "flex",
  flexDirection: "column",
  gap: "4px",
  marginTop: "8px",
};

const projectLinkStyle: StyleMap = {
  textDecoration: "none",
  color: zinc[300],
  transition: "all 200ms ease-in-out",
  display: "block",
};

const contactLinkStyle: StyleMap = {
  textDecoration: "none",
  color: zinc[400],
  transition: "all 200ms ease-in-out",
  display: "block",
};

const nameStyle: StyleMap = {
  color: zinc[100],
  transition: "color 200ms",
};

const descStyle: StyleMap = {
  color: zinc[500],
  transition: "color 200ms",
};

const footerStyle: StyleMap = {
  color: zinc[600],
  fontSize: "12px",
  marginTop: "48px",
};

function projectLink(href: string, name: string, desc: string, indent = false): CrepusNode {
  return {
    kind: "link",
    href,
    style: projectLinkStyle,
    children: [
      {
        kind: "stack",
        style: { display: "block", marginLeft: indent ? "16px" : "0" },
        children: [
          { kind: "text", content: name, style: nameStyle },
          { kind: "text", content: desc, style: descStyle },
        ],
      },
    ],
  };
}

function contactLink(href: string, name: string, desc: string): CrepusNode {
  return {
    kind: "link",
    href,
    style: contactLinkStyle,
    children: [
      {
        kind: "stack",
        style: { display: "block" },
        children: [
          { kind: "text", content: name, style: nameStyle },
          { kind: "text", content: desc, style: descStyle },
        ],
      },
    ],
  };
}

export const pageIr: CrepusIr = {
  version: 1,
  root: [
    {
      kind: "stack",
      axis: "column",
      style: pageStyle,
      children: [
        {
          kind: "stack",
          axis: "column",
          style: containerStyle,
          children: [
            {
              kind: "stack",
              style: { display: "block" },
              children: [
                {
                  kind: "stack",
                  style: { display: "block", ...headingStyle },
                  children: [
                    { kind: "text", content: "the software company of hong kong" },
                  ],
                },
              ],
            },
            {
              kind: "stack",
              style: { display: "block", ...descriptionStyle },
              children: [
                {
                  kind: "text",
                  content:
                    "an independent R&D lab building systems software, runtimes, developer tools, application platforms, and experimental computing platforms.",
                },
              ],
            },
            {
              kind: "stack",
              axis: "column",
              children: [
                {
                  kind: "stack",
                  style: { display: "block", ...sectionLabelStyle },
                  children: [{ kind: "text", content: "PROJECTS" }],
                },
                {
                  kind: "stack",
                  axis: "column",
                  style: projectsStyle,
                  children: [
                    projectLink(
                      "https://crepuscularity.tsc.hk",
                      "crepuscularity",
                      " — write react + rust for hardware accelerated mobile and desktop apps, websites and webextensions, terminal ui and embedded appliances",
                    ),
                    projectLink(
                      "https://github.com/tschk/aurorality",
                      "aurorality",
                      " — transpile crepuscularity to swiftui",
                      true,
                    ),
                    projectLink(
                      "https://crepuscularity.tsc.hk/docs/lite.html",
                      "crepuscularity + aurora lite",
                      " — drop crepuscularity into any electron app or website to get the same ui but native with the same javascript/typescript backend",
                      true,
                    ),
                    projectLink(
                      "https://alpenglow.tsc.hk",
                      "alpenglow",
                      " — Linux operating system that boots in under a second and is smaller than an average picture",
                    ),
                    projectLink(
                      "https://github.com/tschk/soliloquy",
                      "soliloquy",
                      " — the immutable web-native ultraminimal os built on top of alpenglow. spiritual successor to chromeos",
                      true,
                    ),
                    projectLink(
                      "https://inauguration.tsc.hk",
                      "inauguration",
                      " — capability-aware compiler pipeline for deterministic execution and agent-native software",
                    ),
                    projectLink(
                      "https://space.tsc.hk",
                      "space",
                      " — component-based compiler-centric ground-up non-posix operating system",
                    ),
                    projectLink(
                      "https://github.com/tschk/subspace",
                      "subspace",
                      " — statically composed capability-safe embedded operating system",
                      true,
                    ),
                    projectLink(
                      "https://github.com/tschk/equilibrium",
                      "equilibrium",
                      " — cross c compiling language interoperability with one call automagically",
                    ),
                    projectLink(
                      "https://github.com/tschk/eqswift",
                      "eqswift",
                      " — two lines to ffi rust to swift automagically",
                      true,
                    ),
                    projectLink(
                      "https://github.com/tschk/rv8",
                      "rv8",
                      " — browser engine combining Servo rendering with V8 JavaScript",
                    ),
                    projectLink(
                      "https://github.com/tschk/rotary",
                      "rotary",
                      " — pure Rust agent harness engine for hosts, tools, permissions, sessions, and control planes",
                    ),
                    projectLink(
                      "https://github.com/tschk/apollo",
                      "apollo",
                      " — local-first rust ai agent runtime. ~14mb binary, 10+ messaging channels, 20+ llm providers, autonomous coding mode, tool guardrails, plugin system",
                    ),
                    projectLink(
                      "https://github.com/tschk/moonshine",
                      "moonshine",
                      " — ground-up Bun-first hybrid web framework with a signal-only kernel and opt-in compiler, routing, rendering, server, and deployment layers",
                    ),
                    projectLink(
                      "https://github.com/tschk/zkr",
                      "zkr",
                      " — evidence-backed temporal memory engine for personal agents",
                    ),
                    projectLink(
                      "https://github.com/tschk/praefectus",
                      "praefectus",
                      " — provider-neutral, verified computer-use execution for rust",
                    ),
                    projectLink(
                      "https://github.com/tschk/wax",
                      "wax",
                      " — fast homebrew-compatible package manager in rust. compiled, async, parallel installs",
                    ),
                    projectLink(
                      "https://nexnet.tsc.hk",
                      "nexnet",
                      " — local-first peer-to-peer social chat with wallet identity and encrypted messaging",
                    ),
                  ],
                },
              ],
            },
            {
              kind: "stack",
              axis: "column",
              children: [
                {
                  kind: "stack",
                  style: { display: "block", ...sectionLabelStyle },
                  children: [{ kind: "text", content: "CONTACT" }],
                },
                {
                  kind: "stack",
                  axis: "column",
                  style: contactsStyle,
                  children: [
                    contactLink(
                      "https://github.com/tschk",
                      "GitHub",
                      " — https://github.com/tschk",
                    ),
                    contactLink(
                      "https://x.com/tsc_hk",
                      "X",
                      " — @tsc_hk",
                    ),
                    contactLink(
                      "https://instagram.com/thesoftwarecompanyofhongkong",
                      "Instagram",
                      " — @thesoftwarecompanyofhongkong",
                    ),
                    contactLink(
                      "mailto:hello@tsc.hk",
                      "Email",
                      " — hello@tsc.hk",
                    ),
                  ],
                },
              ],
            },
            {
              kind: "stack",
              style: { display: "block", ...footerStyle },
              children: [
                { kind: "text", content: "site built with crepuscularity-moonshine" },
              ],
            },
          ],
        },
      ],
    },
  ],
};
