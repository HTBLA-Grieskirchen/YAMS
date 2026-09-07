"use client";

import { AppShell } from "@/components/layout/app-shell";
import { PageChromeProvider } from "@/components/layout/page-chrome";

export default function AppLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <PageChromeProvider>
      <AppShell>{children}</AppShell>
    </PageChromeProvider>
  );
}
