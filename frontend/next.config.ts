import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactCompiler: true,
  reactStrictMode: true,
  images: {
    unoptimized: true,
  },
  output: "export",
};

export default nextConfig;
