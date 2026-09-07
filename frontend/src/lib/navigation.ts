import type { LucideIcon } from "lucide-react";
import {
  BookOpen,
  CalendarDays,
  Database,
  Home,
  Receipt,
  User,
} from "lucide-react";

export type NavAction = {
  label: string;
  href: string;
};

export type NavItem = {
  id: string;
  label: string;
  href: string;
  icon: LucideIcon;
  recursive?: boolean;
};

export type NavCategory = {
  label: string;
  items: NavItem[];
};

export const paths = {
  home: "/",
  klienten: "/klient",
  klientNeu: "/klient/neu",
  klient: (id: string) => `/klient/${encodeURIComponent(id)}`,
  katalog: "/katalog",
  seminar: "/seminar",
  abrechnung: "/abrechnung",
  objekte: "/objekte",
} as const;

export const sidebarCategories: NavCategory[] = [
  {
    label: "Verwaltung",
    items: [
      {
        id: "klient",
        label: "Klient",
        href: paths.klienten,
        icon: User,
        recursive: true,
      },
      {
        id: "katalog",
        label: "Katalog",
        href: paths.katalog,
        icon: BookOpen,
        recursive: true,
      },
      {
        id: "seminar",
        label: "Seminar",
        href: paths.seminar,
        icon: CalendarDays,
        recursive: true,
      },
    ],
  },
  {
    label: "Workflows",
    items: [
      {
        id: "abrechnung",
        label: "Abrechnung",
        href: paths.abrechnung,
        icon: Receipt,
        recursive: true,
      },
      {
        id: "objekte",
        label: "Objekte",
        href: paths.objekte,
        icon: Database,
        recursive: true,
      },
    ],
  },
];

export const homeNavItem: NavItem = {
  id: "home",
  label: "Home",
  href: paths.klienten,
  icon: Home,
  recursive: false,
};

type BreadcrumbSegment = {
  label: string;
  href?: string;
};

const routeLabels: Record<string, string> = {
  klient: "Klient",
  neu: "Neu",
  katalog: "Katalog",
  seminar: "Seminar",
  abrechnung: "Abrechnung",
  objekte: "Objekte",
};

export function breadcrumbsForPathname(
  pathname: string,
  dynamicLabel?: string,
): BreadcrumbSegment[] {
  const segments = pathname.split("/").filter(Boolean);
  if (segments.length === 0) {
    return [{ label: "Klient", href: paths.klienten }];
  }

  const crumbs: BreadcrumbSegment[] = [];
  let href = "";

  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index] ?? "";
    href += `/${segment}`;
    const isLast = index === segments.length - 1;
    const isUuid =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(
        segment,
      );

    let label = routeLabels[segment] ?? segment;
    if (isUuid && dynamicLabel) {
      label = dynamicLabel;
    } else if (isUuid) {
      label = "Detail";
    }

    crumbs.push({
      label,
      href: isLast ? undefined : href,
    });
  }

  return crumbs;
}

export function navActionsForPathname(pathname: string): NavAction[] {
  if (pathname === paths.klienten) {
    return [{ label: "Neu", href: paths.klientNeu }];
  }

  return [];
}

export function isNavItemActive(
  pathname: string,
  href: string,
  recursive = true,
): boolean {
  if (recursive) {
    return pathname === href || pathname.startsWith(`${href}/`);
  }
  return pathname === href;
}
