import { Search } from "lucide-react";
import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SearchInputProps = InputHTMLAttributes<HTMLInputElement>;

export function SearchInput({ className, ...props }: SearchInputProps) {
  return (
    <div className="relative w-full max-w-xs">
      <Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-zinc-400" />
      <input
        className={cn(
          "h-10 w-full rounded-lg border border-zinc-300 bg-white pr-3 pl-9 text-sm text-zinc-900 placeholder:text-zinc-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-emerald-600 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100",
          className,
        )}
        {...props}
      />
    </div>
  );
}
