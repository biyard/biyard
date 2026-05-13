/** @type {import('next').NextConfig} */
const nextConfig = {
  transpilePackages: ["@biyard/sdk", "@biyard/widget"],
  reactStrictMode: true,
  allowedDevOrigins: ["*.ggernaut.com"],
};

export default nextConfig;
