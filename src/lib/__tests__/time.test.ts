import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { relativeTime } from "$lib/utils/time";
import en from "$lib/i18n/en";

describe("relativeTime", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-03-26T12:00:00Z"));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  const now = Math.floor(new Date("2026-03-26T12:00:00Z").getTime() / 1000);

  it("returns 'just now' for timestamps less than 60 seconds ago", () => {
    expect(relativeTime(now - 30, en)).toBe("just now");
  });

  it("returns minutes for recent timestamps", () => {
    expect(relativeTime(now - 300, en)).toBe("5m ago");
  });

  it("returns hours for timestamps within a day", () => {
    expect(relativeTime(now - 7200, en)).toBe("2h ago");
  });

  it("returns days for timestamps within a week", () => {
    expect(relativeTime(now - 86400 * 3, en)).toBe("3d ago");
  });

  it("returns weeks for timestamps within a month", () => {
    expect(relativeTime(now - 86400 * 14, en)).toBe("2w ago");
  });

  it("returns months for timestamps within a year", () => {
    expect(relativeTime(now - 86400 * 60, en)).toBe("2mo ago");
  });

  it("returns years for old timestamps", () => {
    expect(relativeTime(now - 86400 * 400, en)).toBe("1y ago");
  });
});
