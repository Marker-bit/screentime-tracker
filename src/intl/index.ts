import { locales } from "@/intl/locales";
import { useSettingsStore } from "@/lib/settings";
import en from "./locales/en";
import { Locale } from "@/intl/locales/types";

export function t(key: keyof Locale): string {
  const { settings } = useSettingsStore.getState();

  if (!(settings.language in locales)) {
    return en[key as keyof Locale];
  }

  return locales[settings.language as keyof typeof locales][
    key as keyof Locale
  ];
}
