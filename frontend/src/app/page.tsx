"use client";

import { useAlleHaustiereQuery, useHealthQuery } from "@/api/hooks";
import { useYamsApi } from "@/api/provider";

export default function Home() {
  const { mode, remoteApiBaseUrl, loading: apiBootstrapping, error: apiBootstrapError } =
    useYamsApi();

  const health = useHealthQuery();
  const haustiere = useAlleHaustiereQuery();

  const dataLoading = apiBootstrapping || health.isPending || haustiere.isPending;
  const dataError =
    apiBootstrapError ??
    (health.error ? String(health.error) : null) ??
    (haustiere.error ? String(haustiere.error) : null);

  return (
    <main className="mx-auto flex min-h-screen max-w-3xl flex-col gap-8 p-8 font-sans">
      <header className="space-y-2">
        <h1 className="text-3xl font-semibold tracking-tight">YAMS</h1>
        <p className="text-zinc-600 dark:text-zinc-400">
          Server state via TanStack Query — transport via OpenAPI HTTP or Tauri
          commands depending on deployment mode.
        </p>
      </header>

      <section className="rounded-xl border border-zinc-200 p-6 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Deployment</h2>
        {apiBootstrapping ? (
          <p className="mt-2 text-sm text-zinc-500">Resolving adapter…</p>
        ) : apiBootstrapError ? (
          <p className="mt-2 text-sm text-red-600">{apiBootstrapError}</p>
        ) : (
          <dl className="mt-3 space-y-2 text-sm">
            <div className="flex gap-2">
              <dt className="font-medium">Mode</dt>
              <dd>{mode}</dd>
            </div>
            {remoteApiBaseUrl ? (
              <div className="flex gap-2">
                <dt className="font-medium">API base</dt>
                <dd className="font-mono">{remoteApiBaseUrl}</dd>
              </div>
            ) : null}
            <div className="flex gap-2">
              <dt className="font-medium">Health</dt>
              <dd>{health.data ?? "—"}</dd>
            </div>
          </dl>
        )}
      </section>

      <section className="rounded-xl border border-zinc-200 p-6 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Haustiere</h2>
        {dataLoading ? (
          <p className="mt-2 text-sm text-zinc-500">Loading…</p>
        ) : dataError ? (
          <p className="mt-2 text-sm text-red-600">{dataError}</p>
        ) : (
          <p className="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
            {haustiere.data?.length === 0
              ? "No animals loaded yet."
              : `${haustiere.data?.length} Haustier(e): ${haustiere.data
                  ?.map((h) => h.name)
                  .join(", ")}`}
          </p>
        )}
      </section>
    </main>
  );
}
