export function todayIsoDate(): string {
  return new Date().toISOString().slice(0, 10);
}

/** Convert `datetime-local` value to UTC ISO string for API. */
export function datetimeLocalToIso(value: string): string {
  return new Date(value).toISOString();
}

/** Default `datetime-local` input value (local timezone). */
export function defaultDatetimeLocal(hour: number, minute = 0): string {
  const date = new Date();
  date.setHours(hour, minute, 0, 0);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}
