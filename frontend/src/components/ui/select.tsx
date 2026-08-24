import type { SelectHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SelectProps = SelectHTMLAttributes<HTMLSelectElement>;

export function Select({ className, ...props }: SelectProps) {
  return (
    <select
      className={cn(
        "h-10 w-full rounded-lg border border-zinc-300 bg-white px-3 text-sm text-zinc-900 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-emerald-600 disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100",
        className,
      )}
      {...props}
    />
  );
}
