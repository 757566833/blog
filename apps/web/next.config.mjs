import path, { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { PHASE_PRODUCTION_BUILD, PHASE_PRODUCTION_SERVER } from 'next/constants.js';
const __dirname = dirname(fileURLToPath(import.meta.url));

/** @type {import('next').NextConfig} */
const config = (phase) => {
  let nextConfig = {
    reactStrictMode: true,
    transpilePackages: ["@workspace/ui"],

    // experimental: {

    // },

    outputFileTracingRoot: path.join(__dirname, "../../"),
  }
  if (PHASE_PRODUCTION_SERVER === phase || PHASE_PRODUCTION_BUILD == phase) {
    nextConfig.output = "standalone";
    nextConfig.compiler = { removeConsole: true }
    // nextConfig = withSentryConfig(nextConfig, sentryWebpackPluginOptions);
  }

  return nextConfig;

};

export default config;
