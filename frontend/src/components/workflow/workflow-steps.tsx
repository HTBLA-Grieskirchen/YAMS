import { cn } from "@/lib/cn";

type Step = {
  id: string;
  label: string;
  done: boolean;
  optional?: boolean;
};

type WorkflowStepsProps = {
  steps: Step[];
  currentStepId: string;
};

export function WorkflowSteps({ steps, currentStepId }: WorkflowStepsProps) {
  return (
    <ol className="flex flex-wrap gap-2">
      {steps.map((step, index) => {
        const isCurrent = step.id === currentStepId;
        const isDone = step.done;

        return (
          <li
            key={step.id}
            className={cn(
              "flex items-center gap-2 rounded-full border px-3 py-1 text-xs font-medium",
              isCurrent
                ? "border-emerald-600 bg-emerald-50 text-emerald-900 dark:bg-emerald-950 dark:text-emerald-100"
                : isDone
                  ? "border-zinc-300 text-zinc-600 dark:border-zinc-600 dark:text-zinc-300"
                  : "border-zinc-200 text-zinc-400 dark:border-zinc-800",
            )}
          >
            <span
              className={cn(
                "flex size-5 items-center justify-center rounded-full text-[10px]",
                isCurrent
                  ? "bg-emerald-600 text-white"
                  : isDone
                    ? "bg-zinc-200 text-zinc-700 dark:bg-zinc-700 dark:text-zinc-100"
                    : "bg-zinc-100 text-zinc-400 dark:bg-zinc-800",
              )}
            >
              {index + 1}
            </span>
            {step.label}
            {step.optional ? (
              <span className="text-zinc-400">(optional)</span>
            ) : null}
          </li>
        );
      })}
    </ol>
  );
}
