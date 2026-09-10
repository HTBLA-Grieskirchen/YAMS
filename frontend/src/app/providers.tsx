"use client";

import { QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";

import { YamsApiProvider } from "@/api/provider";
import { createQueryClient } from "@/lib/query-client";

export function Providers({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => createQueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <YamsApiProvider>{children}</YamsApiProvider>
    </QueryClientProvider>
  );
}
