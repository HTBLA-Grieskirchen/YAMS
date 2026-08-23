"use client";

import { YamsApiProvider } from "@/api/provider";

export function Providers({ children }: { children: React.ReactNode }) {
  return <YamsApiProvider>{children}</YamsApiProvider>;
}
