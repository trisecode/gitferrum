import en from "./en";
import es from "./es";
import pt from "./pt";

export type Locale = "en" | "es" | "pt";
export type Translations = typeof en;

const locales: Record<Locale, Translations> = { en, es, pt };

export const localeNames: Record<Locale, string> = {
  en: "English",
  es: "Español",
  pt: "Português",
};

export const availableLocales = Object.keys(locales) as Locale[];

export function getTranslations(locale: Locale): Translations {
  return locales[locale] ?? locales.en;
}

export function detectLocale(): Locale {
  const lang = navigator.language?.toLowerCase() ?? "en";
  if (lang.startsWith("es")) return "es";
  if (lang.startsWith("pt")) return "pt";
  return "en";
}
