"use client";

import Link from "next/link";
import { Mail, MapPin, Phone } from "lucide-react";

import type { Klient } from "@/api/types";
import { HaustierCreateForm } from "@/components/klient/haustier-create-form";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { formatDate } from "@/lib/format";
import { paths } from "@/lib/navigation";

type KlientDetailProps = {
  klient: Klient;
};

export function KlientDetail({ klient }: KlientDetailProps) {
  const label = `${klient.vorname} ${klient.nachname}`;

  return (
    <>
      <SetBreadcrumbLabel label={label} />
      <div className="space-y-6 p-6">
        <Card>
          <CardHeader>
            <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
              <div>
                <CardTitle>
                  {klient.vorname} {klient.nachname}
                </CardTitle>
                <CardDescription>
                  Kundennummer {klient.kundennummer} · Geb.{" "}
                  {formatDate(klient.geburtstag)}
                </CardDescription>
              </div>
              {klient.einwilligung ? (
                <Badge variant="success">Einwilligung erteilt</Badge>
              ) : (
                <Badge variant="error">Keine Einwilligung</Badge>
              )}
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              <InfoRow
                icon={Mail}
                label="E-Mail"
                value={klient.email}
                href={`mailto:${klient.email}`}
              />
              <InfoRow
                icon={Phone}
                label="Mobilnummer"
                value={klient.mobilnummer}
              />
              <InfoRow
                icon={MapPin}
                label="Adresse"
                value={`${klient.adresse.straßeUndHausnummer}, ${klient.adresse.postleitzahl} ${klient.adresse.stadt}, ${klient.adresse.ländercode}`}
              />
            </div>

            <div className="flex flex-wrap gap-2">
              <Link href={paths.leistungForKlient(klient.id)}>
                <Button>Leistung buchen</Button>
              </Link>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Haustiere</CardTitle>
            <CardDescription>
              {klient.haustiere.length} registrierte(s) Haustier(e)
            </CardDescription>
          </CardHeader>
          <CardContent>
            {klient.haustiere.length === 0 ? (
              <p className="text-sm text-zinc-500">Noch keine Haustiere.</p>
            ) : (
              <ul className="mb-4 divide-y divide-zinc-200 dark:divide-zinc-800">
                {klient.haustiere.map((haustier) => (
                  <li key={haustier.id} className="py-3 text-sm">
                    <p className="font-medium">
                      {haustier.name} ({haustier.tierart})
                    </p>
                    <p className="text-zinc-500">
                      Geb. {formatDate(haustier.geburtstag)} ·{" "}
                      {haustier.beschreibung}
                    </p>
                  </li>
                ))}
              </ul>
            )}
            <HaustierCreateForm klientId={klient.id} />
          </CardContent>
        </Card>
      </div>
    </>
  );
}

function InfoRow({
  icon: Icon,
  label,
  value,
  href,
}: {
  icon: typeof Mail;
  label: string;
  value: string;
  href?: string;
}) {
  return (
    <div className="rounded-lg border border-zinc-200 p-4 dark:border-zinc-800">
      <p className="mb-1 flex items-center gap-2 text-xs font-semibold tracking-wide text-zinc-500 uppercase">
        <Icon className="size-4" />
        {label}
      </p>
      {href ? (
        <a
          href={href}
          className="text-sm text-emerald-700 underline-offset-2 hover:underline dark:text-emerald-300"
        >
          {value}
        </a>
      ) : (
        <p className="text-sm">{value}</p>
      )}
    </div>
  );
}
