"use client";

import { Languages, Palette } from "lucide-react";

import { cn } from "@/lib/cn";
import { useTheme } from "@/lib/theme";

export function SidebarFooterControls() {
  return (
    <div className="mt-auto space-y-4 border-t border-zinc-200 pt-6 dark:border-zinc-800">
      <div className="flex gap-2">
        <ThemePicker />
        <LanguagePicker />
      </div>
      <BrandFooter />
    </div>
  );
}

function ThemePicker() {
  const { theme, setTheme } = useTheme();

  return (
    <details className="relative">
      <summary className="flex size-9 cursor-pointer list-none items-center justify-center rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-900 [&::-webkit-details-marker]:hidden">
        <Palette className="size-5" />
      </summary>
      <div className="absolute bottom-full left-0 mb-2 w-36 rounded-lg border border-zinc-200 bg-white p-1 shadow-lg dark:border-zinc-700 dark:bg-zinc-900">
        {(["system", "light", "dark"] as const).map((option) => (
          <button
            key={option}
            type="button"
            className={cn(
              "w-full rounded-md px-3 py-2 text-left text-sm capitalize hover:bg-zinc-100 dark:hover:bg-zinc-800",
              theme === option &&
                "bg-emerald-50 text-emerald-800 dark:bg-emerald-950",
            )}
            onClick={() => setTheme(option)}
          >
            {option}
          </button>
        ))}
      </div>
    </details>
  );
}

function LanguagePicker() {
  return (
    <button
      type="button"
      title="Sprache"
      className="flex size-9 items-center justify-center rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-900"
    >
      <Languages className="size-5" />
    </button>
  );
}

function BrandFooter() {
  return (
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
  );
}
