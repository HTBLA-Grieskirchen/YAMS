/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const yamsKeys = {
  root: ["yams"] as const,
  health: () => [...yamsKeys.root, "health"] as const,
  haustiere: {
    all: () => [...yamsKeys.root, "haustiere"] as const,
    list: () => [...yamsKeys.haustiere.all(), "list"] as const,
    detail: (id: string) => [...yamsKeys.haustiere.all(), "detail", id] as const,
  },
  rechnungen: {
    all: () => [...yamsKeys.root, "rechnungen"] as const,
    byKlient: (klientId: string) =>
      [...yamsKeys.rechnungen.all(), klientId] as const,
  },
};
