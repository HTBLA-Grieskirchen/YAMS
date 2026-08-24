import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

export function Label({
  className,
  ...props
}: HTMLAttributes<HTMLSpanElement>) {
  return (
    <span
      className={cn(
        "text-sm font-medium text-zinc-800 dark:text-zinc-200",
        className,
      )}
      {...props}
    />
  );
}
