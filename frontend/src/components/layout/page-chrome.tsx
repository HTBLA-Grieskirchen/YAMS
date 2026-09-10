"use client";

import {
  createContext,
  type ReactNode,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

type PageChromeContextValue = {
  breadcrumbLabel?: string;
  setBreadcrumbLabel: (label: string | undefined) => void;
};

const PageChromeContext = createContext<PageChromeContextValue | null>(null);

export function PageChromeProvider({ children }: { children: ReactNode }) {
  const [breadcrumbLabel, setBreadcrumbLabel] = useState<string | undefined>();

  const value = useMemo(
    () => ({ breadcrumbLabel, setBreadcrumbLabel }),
    [breadcrumbLabel],
  );

  return (
    <PageChromeContext.Provider value={value}>
      {children}
    </PageChromeContext.Provider>
  );
}

export function usePageChrome() {
  const context = useContext(PageChromeContext);
  if (!context) {
    throw new Error("usePageChrome must be used within PageChromeProvider");
  }
  return context;
}

export function SetBreadcrumbLabel({ label }: { label: string }) {
  const { setBreadcrumbLabel } = usePageChrome();

  useEffect(() => {
    setBreadcrumbLabel(label);
    return () => setBreadcrumbLabel(undefined);
  }, [label, setBreadcrumbLabel]);

  return null;
}
