export function todayIsoDate(): string {
  return toIsoDate(new Date());
}

export function toIsoDate(date: Date): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

export function isoDateFromInstant(iso: string): string {
  return toIsoDate(new Date(iso));
}

export function calendarMonthLabel(date: Date, locale = "de-AT"): string {
  return date.toLocaleDateString(locale, { month: "long", year: "numeric" });
}

export function calendarWeekdayLabels(locale = "de-AT"): string[] {
  const monday = new Date(2024, 0, 1);
  return Array.from({ length: 7 }, (_, index) => {
    const day = new Date(monday);
    day.setDate(monday.getDate() + index);
    return day.toLocaleDateString(locale, { weekday: "short" });
  });
}

export function calendarGridDays(month: Date): Date[] {
  const firstOfMonth = new Date(month.getFullYear(), month.getMonth(), 1);
  const startOffset = (firstOfMonth.getDay() + 6) % 7;
  const gridStart = new Date(firstOfMonth);
  gridStart.setDate(firstOfMonth.getDate() - startOffset);

  return Array.from({ length: 42 }, (_, index) => {
    const day = new Date(gridStart);
    day.setDate(gridStart.getDate() + index);
    return day;
  });
}

export function addMonths(date: Date, amount: number): Date {
  return new Date(date.getFullYear(), date.getMonth() + amount, 1);
}

/** Convert `datetime-local` value to UTC ISO string for API. */
export function datetimeLocalToIso(value: string): string {
  return new Date(value).toISOString();
}

/** Default `datetime-local` input value (local timezone). */
export function defaultDatetimeLocal(hour: number, minute = 0): string {
  return datetimeLocalForDate(todayIsoDate(), hour, minute);
}

export function datetimeLocalForDate(
  isoDate: string,
  hour: number,
  minute = 0,
): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${isoDate}T${pad(hour)}:${pad(minute)}`;
}

export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}
