import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ObjektePanel } from "@/components/workflow/objekte-panel";

export default function ObjektePage() {
  return (
    <div className="space-y-6 p-6">
      <Card className="border-none bg-transparent shadow-none">
        <CardHeader className="px-0 pt-0">
          <CardTitle>Objekte</CardTitle>
          <CardDescription>
            Vollständige Datenübersicht aller Entitäten im System.
          </CardDescription>
        </CardHeader>
      </Card>
      <ObjektePanel />
    </div>
  );
}
