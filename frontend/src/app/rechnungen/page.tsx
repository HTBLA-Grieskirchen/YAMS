import { RechnungenOverview } from "@/components/rechnung/rechnungen-overview";

export default function RechnungenPage() {
  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">Rechnungen</h1>
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          Alle Rechnungen mit PDF-Download.
        </p>
      </div>
      <RechnungenOverview />
    </div>
  );
}
