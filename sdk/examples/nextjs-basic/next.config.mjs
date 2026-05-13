import { fileURLToPath } from "url";
import { dirname, resolve } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  outputFileTracingRoot: resolve(__dirname, "../../"),
  transpilePackages: ["@biyard/sdk", "@biyard/widget"],
  reactStrictMode: true,
  allowedDevOrigins: ["*.ggernaut.com"],
};

export default nextConfig;
