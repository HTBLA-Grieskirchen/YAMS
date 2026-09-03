/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const yamsKeys = {
  root: ["yams"] as const,
  health: () => [...yamsKeys.root, "health"] as const,
  klienten: {
    all: () => [...yamsKeys.root, "klienten"] as const,
    list: () => [...yamsKeys.klienten.all(), "list"] as const,
  },
  haustiere: {
    all: () => [...yamsKeys.root, "haustiere"] as const,
    list: () => [...yamsKeys.haustiere.all(), "list"] as const,
    detail: (id: string) => [...yamsKeys.haustiere.all(), "detail", id] as const,
  },
  produkte: {
    all: () => [...yamsKeys.root, "produkte"] as const,
    list: () => [...yamsKeys.produkte.all(), "list"] as const,
  },
  behandlungen: {
    all: () => [...yamsKeys.root, "behandlungen"] as const,
    list: () => [...yamsKeys.behandlungen.all(), "list"] as const,
  },
  leistungen: {
    all: () => [...yamsKeys.root, "leistungen"] as const,
    list: () => [...yamsKeys.leistungen.all(), "list"] as const,
  },
  rechnungen: {
    all: () => [...yamsKeys.root, "rechnungen"] as const,
    list: () => [...yamsKeys.rechnungen.all(), "list"] as const,
    byKlient: (klientId: string) =>
      [...yamsKeys.rechnungen.all(), klientId] as const,
    pdf: (id: string) => [...yamsKeys.rechnungen.all(), "pdf", id] as const,
  },
  teilnahmebestätigung: {
    pdf: (terminId: string, buchungId: string) =>
      [...yamsKeys.root, "teilnahmebestätigung", terminId, buchungId] as const,
  },
  seminare: {
    all: () => [...yamsKeys.root, "seminare"] as const,
    list: () => [...yamsKeys.seminare.all(), "list"] as const,
    detail: (id: string) => [...yamsKeys.seminare.all(), "detail", id] as const,
  },
  seminarTermine: {
    all: () => [...yamsKeys.root, "seminar-termine"] as const,
    list: () => [...yamsKeys.seminarTermine.all(), "list"] as const,
    detail: (id: string) =>
      [...yamsKeys.seminarTermine.all(), "detail", id] as const,
    umsatz: (id: string) =>
      [...yamsKeys.seminarTermine.all(), "umsatz", id] as const,
  },
  seminarPrognose: (stichtag: string) =>
    [...yamsKeys.root, "seminar-prognose", stichtag] as const,
};
