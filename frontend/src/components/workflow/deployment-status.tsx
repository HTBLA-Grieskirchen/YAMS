"use client";

import { useHealthQuery } from "@/api/hooks";
import { useYamsApi } from "@/api/provider";
import { Alert } from "@/components/ui/alert";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export function DeploymentStatus() {
  const {
    mode,
    remoteApiBaseUrl,
    loading: apiBootstrapping,
    error: apiBootstrapError,
  } = useYamsApi();
  const health = useHealthQuery();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Deployment</CardTitle>
        <CardDescription>
          Transport adapter and backend health for this session.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {apiBootstrapping ? (
          <p className="text-sm text-zinc-500">Resolving adapter…</p>
        ) : apiBootstrapError ? (
          <Alert variant="error">{apiBootstrapError}</Alert>
        ) : (
          <dl className="grid gap-3 text-sm sm:grid-cols-2">
            <div>
              <dt className="font-medium text-zinc-500">Mode</dt>
              <dd className="mt-0.5 font-mono">{mode}</dd>
            </div>
            {remoteApiBaseUrl ? (
              <div>
                <dt className="font-medium text-zinc-500">API base</dt>
                <dd className="mt-0.5 font-mono text-xs">{remoteApiBaseUrl}</dd>
              </div>
            ) : null}
            <div>
              <dt className="font-medium text-zinc-500">Health</dt>
              <dd className="mt-0.5">
                {health.isPending ? "…" : (health.data ?? "—")}
              </dd>
            </div>
          </dl>
        )}
      </CardContent>
    </Card>
  );
}
