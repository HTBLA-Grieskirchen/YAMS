import { useYamsApi } from "../provider";

/** YamsApi context with a boolean gate for TanStack Query `enabled`. */
export function useYamsApiReady() {
  const context = useYamsApi();
  const isReady = !context.loading && context.api !== null;

  return {
    ...context,
    isReady,
    api: context.api,
  };
}
