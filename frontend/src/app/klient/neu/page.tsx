import { KlientRegisterForm } from "@/components/klient/klient-register-form";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function KlientNeuPage() {
  return (
    <div className="p-6">
      <Card>
        <CardHeader>
          <CardTitle>Klient anlegen</CardTitle>
          <CardDescription>
            Registrierung wie in der Legacy-Client-Ansicht.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <KlientRegisterForm />
        </CardContent>
      </Card>
    </div>
  );
}
