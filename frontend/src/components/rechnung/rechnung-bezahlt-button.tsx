"use client";

import { CheckCircle2 } from "lucide-react";
import { useState } from "react";

import { useRechnungAlsBezahltMarkierenMutation } from "@/api/hooks";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { todayIsoDate } from "@/lib/dates";

type RechnungBezahltButtonProps = {
  rechnungId: string;
};

export function RechnungBezahltButton({ rechnungId }: RechnungBezahltButtonProps) {
  const mutation = useRechnungAlsBezahltMarkierenMutation();
  const [bezahltDatum, setBezahltDatum] = useState(todayIsoDate());

  async function handleMarkPaid() {
    await mutation.mutateAsync({
      rechnungId,
      body: { bezahltDatum },
    });
  }

  return (
    <div className="flex flex-col gap-1">
      <div className="flex flex-wrap items-center gap-2">
        <Input
          type="date"
          value={bezahltDatum}
          onChange={(event) => setBezahltDatum(event.target.value)}
          className="h-8 w-36"
        />
        <Button
          type="button"
          variant="secondary"
          size="sm"
          disabled={mutation.isPending}
          onClick={handleMarkPaid}
        >
          <CheckCircle2 className="size-4" />
          {mutation.isPending ? "Speichern…" : "Bezahlt"}
        </Button>
      </div>
      {mutation.error ? (
        <span className="text-xs text-red-600">{String(mutation.error)}</span>
      ) : null}
    </div>
  );
}
