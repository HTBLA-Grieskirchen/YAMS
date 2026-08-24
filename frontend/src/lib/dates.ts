export function todayIsoDate(): string {
  return new Date().toISOString().slice(0, 10);
}
