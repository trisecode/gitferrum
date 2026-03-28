import type { Translations } from "$lib/i18n";

export function relativeTime(timestamp: number, t: Translations): string {
  const now = Math.floor(Date.now() / 1000);
  const diff = now - timestamp;

  if (diff < 60) return t.justNow;
  if (diff < 3600) return t.minutesAgo(Math.floor(diff / 60));
  if (diff < 86400) return t.hoursAgo(Math.floor(diff / 3600));
  if (diff < 604800) return t.daysAgo(Math.floor(diff / 86400));
  if (diff < 2592000) return t.weeksAgo(Math.floor(diff / 604800));
  if (diff < 31536000) return t.monthsAgo(Math.floor(diff / 2592000));
  return t.yearsAgo(Math.floor(diff / 31536000));
}
