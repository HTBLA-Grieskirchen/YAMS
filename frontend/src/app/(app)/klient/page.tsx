import { KlientTable } from "@/components/klient/klient-table";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function KlientenPage() {
  return (
    <div className="p-6">
      <Card className="border-none bg-transparent shadow-none">
        <CardHeader className="px-0 pt-0">
          <CardTitle>Klienten</CardTitle>
          <CardDescription>
            Übersicht aller Klienten mit expandierbaren Details und Haustieren.
          </CardDescription>
        </CardHeader>
        <CardContent className="px-0">
          <KlientTable />
        </CardContent>
      </Card>
    </div>
  );
}
