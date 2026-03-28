import { describe, it, expect } from "vitest";
import en from "$lib/i18n/en";
import es from "$lib/i18n/es";
import pt from "$lib/i18n/pt";
import { availableLocales, getTranslations } from "$lib/i18n";
import { relativeTime } from "$lib/utils/time";

describe("i18n", () => {
  it("has 3 available locales", () => {
    expect(availableLocales).toEqual(["en", "es", "pt"]);
  });

  it("all locales have the same keys", () => {
    const enKeys = Object.keys(en).sort();
    const esKeys = Object.keys(es).sort();
    const ptKeys = Object.keys(pt).sort();
    expect(esKeys).toEqual(enKeys);
    expect(ptKeys).toEqual(enKeys);
  });

  it("all function keys return strings", () => {
    for (const locale of [en, es, pt]) {
      expect(typeof locale.stageNFiles(1)).toBe("string");
      expect(typeof locale.stageNFiles(5)).toBe("string");
      expect(typeof locale.nChanges(0)).toBe("string");
      expect(typeof locale.pullTooltip(3)).toBe("string");
      expect(typeof locale.pushTooltip(1)).toBe("string");
      expect(typeof locale.repoOpened("test")).toBe("string");
      expect(typeof locale.repoCloned("test")).toBe("string");
      expect(typeof locale.checkedOut("main")).toBe("string");
      expect(typeof locale.branchCreated("feat")).toBe("string");
      expect(typeof locale.browsingDetached("origin/main")).toBe("string");
      expect(typeof locale.filesChanged(3)).toBe("string");
      expect(typeof locale.minutesAgo(5)).toBe("string");
      expect(typeof locale.hoursAgo(2)).toBe("string");
      expect(typeof locale.daysAgo(1)).toBe("string");
      expect(typeof locale.weeksAgo(3)).toBe("string");
      expect(typeof locale.monthsAgo(6)).toBe("string");
      expect(typeof locale.yearsAgo(1)).toBe("string");
    }
  });

  it("getTranslations returns correct locale", () => {
    expect(getTranslations("es").appSubtitle).toBe("Cliente Git de alto rendimiento");
    expect(getTranslations("pt").appSubtitle).toBe("Cliente Git de alto desempenho");
    expect(getTranslations("en").appSubtitle).toBe("High-performance Git client");
  });

  it("relativeTime works with Spanish translations", () => {
    const now = Math.floor(Date.now() / 1000);
    expect(relativeTime(now - 30, es)).toBe("ahora");
    expect(relativeTime(now - 300, es)).toContain("hace");
  });

  it("relativeTime works with Portuguese translations", () => {
    const now = Math.floor(Date.now() / 1000);
    expect(relativeTime(now - 30, pt)).toBe("agora");
    expect(relativeTime(now - 300, pt)).toContain("atrás");
  });

  it("singular vs plural works correctly", () => {
    expect(en.stageNFiles(1)).toBe("Stage 1 file");
    expect(en.stageNFiles(3)).toBe("Stage 3 files");
    expect(es.stageNFiles(1)).toBe("Preparar 1 archivo");
    expect(es.stageNFiles(3)).toBe("Preparar 3 archivos");
    expect(en.filesChanged(1)).toBe("1 file changed");
    expect(en.filesChanged(5)).toBe("5 files changed");
  });
});
