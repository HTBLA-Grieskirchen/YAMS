"use client";

import { useEffect, useState } from "react";

import { useYamsApi } from "@/api/provider";
import type { Haustier } from "@/api/types";

export default function Home() {
  const { api, mode, remoteApiBaseUrl, loading, error } = useYamsApi();
  const [haustiere, setHaustiere] = useState<Haustier[]>([]);
  const [fetchError, setFetchError] = useState<string | null>(null);

  useEffect(() => {
    if (!api) {
      return;
    }

    let cancelled = false;

    api
      .alleHaustiere()
      .then((data) => {
        if (!cancelled) {
          setHaustiere(data);
          setFetchError(null);
        }
      })
      .catch((err) => {
        if (!cancelled) {
          setFetchError(String(err));
        }
      });

    return () => {
      cancelled = true;
    };
  }, [api]);

  return (
    <main className="mx-auto flex min-h-screen max-w-3xl flex-col gap-8 p-8 font-sans">
      <header className="space-y-2">
        <h1 className="text-3xl font-semibold tracking-tight">YAMS</h1>
        <p className="text-zinc-600 dark:text-zinc-400">
          Shared API layer — OpenAPI HTTP in remote mode, Tauri commands in
          embedded mode.
        </p>
      </header>

      <section className="rounded-xl border border-zinc-200 p-6 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Deployment</h2>
        {loading ? (
          <p className="mt-2 text-sm text-zinc-500">Resolving adapter…</p>
        ) : error ? (
          <p className="mt-2 text-sm text-red-600">{error}</p>
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
          </dl>
        )}
      </section>

      <section className="rounded-xl border border-zinc-200 p-6 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Haustiere</h2>
        {fetchError ? (
          <p className="mt-2 text-sm text-red-600">{fetchError}</p>
        ) : (
          <p className="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
            {haustiere.length === 0
              ? "No animals loaded yet."
              : `${haustiere.length} Haustier(e): ${haustiere
                  .map((h) => h.name)
                  .join(", ")}`}
          </p>
        )}
      </section>
    </main>
  );
}
