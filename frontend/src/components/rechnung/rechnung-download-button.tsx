"use client";

import { Download } from "lucide-react";
import { useState } from "react";

import { useYamsApiReady } from "@/api/hooks";
import { Button } from "@/components/ui/button";
import { downloadBlob } from "@/lib/dates";

type RechnungDownloadButtonProps = {
  rechnungId: string;
  rechnungsnummer: number;
  variant?: "primary" | "ghost";
};

export function RechnungDownloadButton({
  rechnungId,
  rechnungsnummer,
  variant = "ghost",
}: RechnungDownloadButtonProps) {
  const { api } = useYamsApiReady();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleDownload() {
    if (!api) return;
    setLoading(true);
    setError(null);
    try {
      const blob = await api.rechnungPdf(rechnungId);
      downloadBlob(blob, `rechnung-${rechnungsnummer}.pdf`);
    } catch (downloadError) {
      setError(String(downloadError));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="inline-flex flex-col items-start gap-1">
      <Button
        type="button"
        variant={variant}
        size="sm"
        disabled={loading || !api}
        onClick={handleDownload}
      >
        <Download className="size-4" />
        {loading ? "Lade…" : "PDF"}
      </Button>
      {error ? <span className="text-xs text-red-600">{error}</span> : null}
    </div>
  );
}
