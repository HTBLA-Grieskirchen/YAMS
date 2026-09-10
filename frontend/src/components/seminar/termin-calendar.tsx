"use client";

import { ChevronLeft, ChevronRight } from "lucide-react";
import { useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/cn";
import {
  addMonths,
  calendarGridDays,
  calendarMonthLabel,
  calendarWeekdayLabels,
  isoDateFromInstant,
  todayIsoDate,
  toIsoDate,
} from "@/lib/dates";

export type TerminDaySummary = {
  total: number;
  visible: number;
  hidden: number;
};

type TerminCalendarProps = {
  eventsByDay: Map<string, TerminDaySummary>;
  selectedDate: string | null;
  onSelectDate: (date: string | null) => void;
  onPlanTermin?: (date: string) => void;
};

export function TerminCalendar({
  eventsByDay,
  selectedDate,
  onSelectDate,
  onPlanTermin,
}: TerminCalendarProps) {
  const [visibleMonth, setVisibleMonth] = useState(() => {
    const initial = selectedDate
      ? new Date(`${selectedDate}T12:00:00`)
      : new Date();
    return new Date(initial.getFullYear(), initial.getMonth(), 1);
  });

  const today = todayIsoDate();
  const weekdayLabels = useMemo(() => calendarWeekdayLabels(), []);
  const gridDays = useMemo(
    () => calendarGridDays(visibleMonth),
    [visibleMonth],
  );

  function handleSelectDay(isoDate: string) {
    onSelectDate(selectedDate === isoDate ? null : isoDate);
  }

  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between gap-2">
          <div>
            <CardTitle>Kalender</CardTitle>
            <CardDescription>
              Termine nach Datum — ausgeblendete Termine erscheinen blasser.
            </CardDescription>
          </div>
          <div className="flex items-center gap-1">
            <button
              type="button"
              aria-label="Vorheriger Monat"
              onClick={() => setVisibleMonth((month) => addMonths(month, -1))}
              className="rounded-lg p-2 text-zinc-600 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800"
            >
              <ChevronLeft className="size-4" />
            </button>
            <span className="min-w-36 text-center text-sm font-medium capitalize">
              {calendarMonthLabel(visibleMonth)}
            </span>
            <button
              type="button"
              aria-label="Nächster Monat"
              onClick={() => setVisibleMonth((month) => addMonths(month, 1))}
              className="rounded-lg p-2 text-zinc-600 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800"
            >
              <ChevronRight className="size-4" />
            </button>
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-4">
        <div className="grid grid-cols-7 gap-px overflow-hidden rounded-lg bg-zinc-200 text-center text-xs font-medium text-zinc-500 dark:bg-zinc-800">
          {weekdayLabels.map((label) => (
            <div key={label} className="bg-white py-2 dark:bg-zinc-950">
              {label}
            </div>
          ))}

          {gridDays.map((day) => {
            const isoDate = toIsoDate(day);
            const inCurrentMonth = day.getMonth() === visibleMonth.getMonth();
            const summary = eventsByDay.get(isoDate);
            const isSelected = selectedDate === isoDate;
            const isToday = isoDate === today;
            const hasVisible = (summary?.visible ?? 0) > 0;
            const hasHiddenOnly = (summary?.total ?? 0) > 0 && !hasVisible;

            return (
              <button
                key={isoDate}
                type="button"
                onClick={() => handleSelectDay(isoDate)}
                className={cn(
                  "relative flex min-h-14 flex-col items-center justify-start py-1.5 transition-colors hover:bg-zinc-50 dark:hover:bg-zinc-900",
                  !inCurrentMonth
                    ? "bg-white text-zinc-400 dark:bg-zinc-950 dark:text-zinc-600"
                    : "bg-white dark:bg-zinc-950",
                  hasVisible &&
                    inCurrentMonth &&
                    "bg-emerald-50 dark:bg-emerald-950/30",
                  hasHiddenOnly &&
                    inCurrentMonth &&
                    "bg-emerald-50/40 dark:bg-emerald-950/15",
                  isSelected &&
                    "ring-2 ring-inset ring-emerald-600 dark:ring-emerald-500",
                  isToday &&
                    !isSelected &&
                    "font-semibold text-emerald-700 dark:text-emerald-400",
                )}
              >
                <time
                  dateTime={isoDate}
                  className={cn(
                    "flex size-7 items-center justify-center rounded-full text-sm",
                    isSelected && "bg-emerald-600 font-semibold text-white",
                    isToday &&
                      !isSelected &&
                      "bg-emerald-100 dark:bg-emerald-950/50",
                  )}
                >
                  {day.getDate()}
                </time>

                {summary && summary.total > 0 ? (
                  <span
                    className={cn(
                      "mt-1 inline-flex min-w-5 items-center justify-center rounded-full px-1.5 py-0.5 text-[10px] font-semibold leading-none",
                      hasVisible
                        ? "bg-emerald-600 text-white"
                        : "bg-emerald-600/35 text-emerald-900 dark:bg-emerald-500/25 dark:text-emerald-100",
                      hasHiddenOnly && "opacity-60",
                    )}
                    title={
                      summary.hidden > 0
                        ? `${summary.visible} sichtbar, ${summary.hidden} ausgeblendet`
                        : `${summary.total} Termin(e)`
                    }
                  >
                    {summary.total}
                  </span>
                ) : null}
              </button>
            );
          })}
        </div>

        <Button
          type="button"
          className="w-full"
          disabled={!selectedDate}
          onClick={() => selectedDate && onPlanTermin?.(selectedDate)}
        >
          {selectedDate
            ? `Termin am ${formatSelectedDate(selectedDate)} planen`
            : "Datum wählen, um Termin zu planen"}
        </Button>
      </CardContent>
    </Card>
  );
}

function formatSelectedDate(isoDate: string): string {
  return new Date(`${isoDate}T12:00:00`).toLocaleDateString("de-AT", {
    day: "numeric",
    month: "long",
    year: "numeric",
  });
}

export function buildTerminEventsByDay(
  termine: { id: string; beginn: string }[],
  visibleIds: Set<string>,
): Map<string, TerminDaySummary> {
  const byDay = new Map<string, TerminDaySummary>();

  for (const termin of termine) {
    const dayKey = isoDateFromInstant(termin.beginn);
    const current = byDay.get(dayKey) ?? { total: 0, visible: 0, hidden: 0 };
    current.total += 1;
    if (visibleIds.has(termin.id)) {
      current.visible += 1;
    } else {
      current.hidden += 1;
    }
    byDay.set(dayKey, current);
  }

  return byDay;
}
