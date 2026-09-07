"use client";

import { AppShell } from "@/components/layout/app-shell";
import { PageChromeProvider } from "@/components/layout/page-chrome";
import { ThemeProvider } from "@/lib/theme";

export default function AppLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ThemeProvider>
      <PageChromeProvider>
        <AppShell>{children}</AppShell>
      </PageChromeProvider>
    </ThemeProvider>
  );
}
