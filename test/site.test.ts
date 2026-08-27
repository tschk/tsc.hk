import { describe, expect, test } from "bun:test";
import { createBunServer } from "@tschk/moonshine-deploy-bun";
import { fetch, publicDir } from "../src/app";
import { pageIr, telekinesisIr } from "../src/ir";

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
      expect(html).toContain("telekinesis");
      expect(html).toContain("https://telekinesis.tsc.hk");
      expect(html).toContain(
        "AI coding agent CLI + TUI on the rotary harness. minimal, fast, typed event boundary.",
      );
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

describe("generated View IR", () => {
  test("is IR_VERSION 7 and carries style.id for #tsc-heading", () => {
    expect(pageIr.version).toBe(7);
    expect(JSON.stringify(pageIr)).toContain('"id":"tsc-heading"');
  });

  test("rendered HTML binds id=tsc-heading", async () => {
    const server = createBunServer({ fetch, port: 0, staticDir: publicDir });
    try {
      const res = await fetch(new Request(`${server.url.origin}/`));
      const html = await res.text();
      expect(html).toContain('id="tsc-heading"');
    } finally {
      await server.stop(true);
    }
  });
});

describe("telekinesis page", () => {
  test("GET /telekinesis returns 200 with documented facts", async () => {
    const server = createBunServer({ fetch, port: 0, staticDir: publicDir });
    try {
      const res = await fetch(new Request(`${server.url.origin}/telekinesis`));
      expect(res.status).toBe(200);
      expect(res.headers.get("content-type")).toContain("text/html");
      const html = await res.text();
      expect(html).toContain("telekinesis");
      expect(html).toContain(
        "AI coding agent CLI + TUI. Powered by the rotary (rx4) harness engine and crepuscularity-tui.",
      );
      expect(html).toContain("minimal, fast, typed event boundary");
      expect(html).toContain("rotary owns the loop");
      expect(html).toContain("pi protocol compat layer");
      expect(html).toContain("cd ui/tui &amp;&amp; cargo build --release");
      expect(html).toContain("ui/tui/target/release/tk");
      expect(html).toContain("tk login grok");
      expect(html).toContain("XAI_API_KEY=... tk");
      expect(html).toContain("https://github.com/semitechnological/telekinesis");
      expect(html).toContain("https://github.com/tschk/rotary");
      expect(html).toContain("MPL-2.0");
      expect(html).toContain("telekinesis — tsc.hk");
      expect(html).toContain('id="tk-heading"');
      expect(html).not.toContain("style=");
    } finally {
      await server.stop(true);
    }
  });

  test("is IR_VERSION 7 and carries style.id for #tk-heading", () => {
    expect(telekinesisIr.version).toBe(7);
    expect(JSON.stringify(telekinesisIr)).toContain('"id":"tk-heading"');
  });
});
