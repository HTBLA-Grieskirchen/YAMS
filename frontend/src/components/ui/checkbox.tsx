import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type CheckboxProps = Omit<InputHTMLAttributes<HTMLInputElement>, "type"> & {
  label: string;
};

export function Checkbox({ className, label, id, ...props }: CheckboxProps) {
  const inputId = id ?? label;

  return (
    <label
      className={cn(
        "flex items-center gap-2 text-sm text-zinc-800 dark:text-zinc-200",
        className,
      )}
      htmlFor={inputId}
    >
      <input
        id={inputId}
        type="checkbox"
        className="size-4 rounded border-zinc-300 text-emerald-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-emerald-600 dark:border-zinc-600"
        {...props}
      />
      {label}
    </label>
  );
}
