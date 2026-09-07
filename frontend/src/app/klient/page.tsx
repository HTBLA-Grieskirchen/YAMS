import Link from "next/link";

import { KlientTable } from "@/components/klient/klient-table";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { paths } from "@/lib/navigation";

export default function KlientenPage() {
  return (
    <div className="p-6">
      <Card className="border-none bg-transparent shadow-none">
        <CardHeader className="flex flex-row items-start justify-between gap-4 px-0 pt-0">
          <div>
            <CardTitle>Klienten</CardTitle>
            <CardDescription>
              Übersicht aller Klienten mit expandierbaren Details und Haustieren.
            </CardDescription>
          </div>
          <Link href={paths.klientNeu}>
            <Button size="sm" variant="primary">
              Neu
            </Button>
          </Link>
        </CardHeader>
        <CardContent className="px-0">
          <KlientTable />
        </CardContent>
      </Card>
    </div>
  );
}
