import { type Locale, type Translations, getTranslations } from "$lib/i18n";

class I18nStore {
  locale = $state<Locale>("en");
  t = $derived(getTranslations(this.locale));

  setLocale(locale: Locale) {
    this.locale = locale;
    localStorage.setItem("gitferrum-locale", locale);
  }

  constructor() {
    const saved = localStorage.getItem("gitferrum-locale") as Locale | null;
    if (saved && ["en", "es", "pt"].includes(saved)) {
      this.locale = saved;
    }
  }
}

export const i18n = new I18nStore();
