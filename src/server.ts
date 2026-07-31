import { join } from "node:path";
import { createBunServer } from "@tschk/moonshine-deploy-bun";
import { createRequestHandler } from "@tschk/moonshine-server";
import { pageIr } from "./ir";
import { renderer } from "./renderer";

const root = import.meta.dir;
const publicDir = join(root, "..", "public");

const route = {
  id: "home",
  path: "/",
  file: "src/server.ts",
  mode: "static" as const,
  runtime: "bun" as const,
  decision: "static",
  clientEntries: [] as string[],
};

const fetch = createRequestHandler({
  routes: [route],
  modules: {
    [route.id]: {
      loader: () => pageIr,
    },
  },
  renderer,
  staticDir: publicDir,
});

const server = createBunServer({
  fetch,
  port: Number(process.env.PORT) || 4000,
  staticDir: publicDir,
});

console.log(`tsc.hk → ${server.url.origin}`);
