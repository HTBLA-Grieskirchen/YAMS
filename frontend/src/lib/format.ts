export function formatDate(value: string): string {
  const date = new Date(`${value}T00:00:00`);
  if (Number.isNaN(date.getTime())) {
    return value;
  }
  return date.toLocaleDateString("de-AT");
}

export function matchesSearchQuery(
  query: string,
  values: Array<string | number | boolean | null | undefined>,
): boolean {
  const segments = query
    .trim()
    .split(/[\s,]+/)
    .map((segment) => segment.trim().toLowerCase())
    .filter(Boolean);

  if (segments.length === 0) {
    return true;
  }

  const haystack = values
    .filter((value) => value !== null && value !== undefined)
    .map((value) => String(value).toLowerCase())
    .join(" ");

  return segments.every((segment) => haystack.includes(segment));
}
