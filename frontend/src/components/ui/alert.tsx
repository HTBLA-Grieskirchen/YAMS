import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type AlertVariant = "info" | "success" | "error";

type AlertProps = HTMLAttributes<HTMLDivElement> & {
  variant?: AlertVariant;
};

const variantClasses: Record<AlertVariant, string> = {
  info: "border-zinc-200 bg-zinc-50 text-zinc-800 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-200",
  success:
    "border-emerald-200 bg-emerald-50 text-emerald-900 dark:border-emerald-900 dark:bg-emerald-950 dark:text-emerald-100",
  error:
    "border-red-200 bg-red-50 text-red-900 dark:border-red-900 dark:bg-red-950 dark:text-red-100",
};

export function Alert({
  className,
  variant = "info",
  ...props
}: AlertProps) {
  return (
    <div
      className={cn(
        "rounded-lg border px-4 py-3 text-sm",
        variantClasses[variant],
        className,
      )}
      {...props}
    />
  );
}
