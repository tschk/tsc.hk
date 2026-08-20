import { join } from "node:path";
import { createRequestHandler } from "@tschk/moonshine-server";
import type { RouteArtifact } from "@tschk/moonshine-framework";
import { pageIr, telekinesisIr } from "./ir";
import { renderer } from "./renderer";

export const publicDir = join(import.meta.dir, "..", "public");

export const homeRoute: RouteArtifact = {
  id: "home",
  path: "/",
  file: "src/server.ts",
  mode: "static",
  runtime: "bun",
  decision: "static",
  clientEntries: [],
};

export const telekinesisRoute: RouteArtifact = {
  id: "telekinesis",
  path: "/telekinesis",
  file: "src/server.ts",
  mode: "static",
  runtime: "bun",
  decision: "static",
  clientEntries: [],
};

export const routes = [homeRoute, telekinesisRoute];

export const fetch = createRequestHandler({
  routes,
  modules: {
    [homeRoute.id]: {
      loader: () => pageIr,
    },
    [telekinesisRoute.id]: {
      loader: () => telekinesisIr,
    },
  },
  renderer,
  staticDir: publicDir,
});
