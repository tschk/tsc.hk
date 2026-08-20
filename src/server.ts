import { createBunServer } from "@tschk/moonshine-deploy-bun";
import { fetch, publicDir } from "./app";

const server = createBunServer({
  fetch,
  port: Number(process.env.PORT) || 4000,
  staticDir: publicDir,
});

console.log(`tsc.hk → ${server.url.origin}`);
