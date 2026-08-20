import type { CrepusIr } from "@tschk/crepus-moonshine";
import generated from "./generated/view-ir.json";
import telekinesisGenerated from "./generated/telekinesis-ir.json";

/** Lowered ahead of build by the native Rust parser (`ir-gen`), not at runtime. */
export const pageIr: CrepusIr = generated as unknown as CrepusIr;
export const telekinesisIr: CrepusIr = telekinesisGenerated as unknown as CrepusIr;
