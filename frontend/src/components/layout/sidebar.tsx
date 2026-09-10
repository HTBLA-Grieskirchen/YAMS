"use client";

import Link from "next/link";
import { cn } from "@/lib/cn";
import type { NavCategory, NavItem } from "@/lib/navigation";
import { isNavItemActive } from "@/lib/navigation";

type SidebarProps = {
  categories: NavCategory[];
  homeItem: NavItem;
  pathname: string;
};

export function Sidebar({ categories, homeItem, pathname }: SidebarProps) {
  const HomeIcon = homeItem.icon;
  const homeActive = isNavItemActive(
    pathname,
    homeItem.href,
    homeItem.recursive,
  );

  return (
    <aside
      className={cn(
        "fixed inset-y-0 left-0 z-50 flex w-64 flex-col border-r border-zinc-200 bg-white transition-transform dark:border-zinc-800 dark:bg-zinc-950",
        "-translate-x-full lg:static lg:translate-x-0",
        "peer-checked:translate-x-0",
      )}
    >
      <div className="flex h-full flex-col overflow-y-auto px-4 py-6">
        <div className="mb-8 px-2">
          <Link
            href={homeItem.href}
            className={cn(
              "flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
              homeActive
                ? "bg-emerald-600 text-white"
                : "text-zinc-700 hover:bg-zinc-100 dark:text-zinc-200 dark:hover:bg-zinc-900",
            )}
          >
            <HomeIcon className="size-5" />
            {homeItem.label}
          </Link>
        </div>

        <div className="flex-1 space-y-8">
          {categories.map((category) => (
            <div key={category.label} className="space-y-2">
              <h3 className="px-2 text-xs font-semibold tracking-wider text-zinc-500 uppercase">
                {category.label}
              </h3>
              <div className="space-y-1">
                {category.items.map((item) => (
                  <SidebarLink key={item.id} item={item} pathname={pathname} />
                ))}
              </div>
            </div>
          ))}
        </div>

        <BrandFooter />
      </div>
    </aside>
  );
}

function SidebarLink({ item, pathname }: { item: NavItem; pathname: string }) {
  const Icon = item.icon;
  const active = isNavItemActive(pathname, item.href, item.recursive);

  return (
    <Link
      href={item.href}
      className={cn(
        "flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors",
        active
          ? "bg-emerald-50 font-semibold text-emerald-900 dark:bg-emerald-950 dark:text-emerald-100"
          : "text-zinc-700 hover:bg-zinc-100 dark:text-zinc-200 dark:hover:bg-zinc-900",
      )}
    >
      <Icon className="size-5" />
      {item.label}
    </Link>
  );
}

function BrandFooter() {
  return (
    <div className="mt-auto border-t border-zinc-200 pt-6 dark:border-zinc-800">
      <div className="px-2">
        <div className="mb-2 flex size-12 items-center justify-center rounded-xl bg-emerald-600 text-lg font-bold text-white">
          EP
        </div>
        <div className="text-xs leading-tight text-zinc-500">
          <p className="font-semibold text-zinc-800 dark:text-zinc-200">
            Energetik Sabine Petschl
          </p>
          <p>Wohlfühlen für Mensch und Tier</p>
        </div>
      </div>
    </div>
  );
}
