import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

// We need to test the store in a way that Svelte 5 runes work
// Since runes only work inside Svelte components or .svelte.ts files,
// we test the store behavior through its public interface

describe("ToastStore", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("can be imported", async () => {
    const { toastStore } = await import("$lib/stores/toast.svelte");
    expect(toastStore).toBeDefined();
  });
});
