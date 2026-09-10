import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";
import { Label } from "./label";

type FieldProps = HTMLAttributes<HTMLDivElement> & {
  label: string;
  hint?: string;
  error?: string;
};

export function Field({
  label,
  hint,
  error,
  className,
  children,
  ...props
}: FieldProps) {
  return (
    <div className={cn("space-y-1.5", className)} {...props}>
      <Label>{label}</Label>
      {children}
      {hint ? (
        <p className="text-xs text-zinc-500 dark:text-zinc-400">{hint}</p>
      ) : null}
      {error ? <p className="text-xs text-red-600">{error}</p> : null}
    </div>
  );
}
