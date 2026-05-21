import { defineConfig } from "tsup";

export default defineConfig([
  // Library build — consumed via `import { defineBiyardClaim } from "@biyard/widget"`.
  // External `@biyard/sdk` so npm consumers dedupe.
  {
    entry: { index: "src/index.ts" },
    format: ["esm", "cjs"],
    dts: true,
    sourcemap: true,
    clean: true,
    target: "es2020",
    splitting: false,
    external: ["@biyard/sdk"],
  },
  // CDN build — single self-contained ESM bundle that auto-registers
  // <biyard-claim>. Inlines `@biyard/sdk` and `ethers`.
  {
    entry: { cdn: "src/cdn.ts" },
    format: ["esm"],
    minify: true,
    sourcemap: true,
    target: "es2020",
    splitting: false,
    noExternal: [/.*/],
  },
]);
