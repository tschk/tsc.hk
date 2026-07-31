import { join } from "node:path";
import { parseCrepus, type CrepusIr } from "@tschk/crepus-moonshine";

const source = await Bun.file(
  join(import.meta.dir, "..", "index.crepus"),
).text();

/** Parsed once at startup by the Rust parser (via WASM), not rebuilt per request. */
export const pageIr: CrepusIr = parseCrepus(source);
