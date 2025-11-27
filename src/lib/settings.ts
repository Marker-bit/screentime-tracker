import { create } from "zustand";
import { createJSONStorage, persist, StateStorage } from "zustand/middleware";
import { load } from "@tauri-apps/plugin-store";

interface SettingsState {
  disableAnimation: boolean;
  password: string | null;
  parentEmail: string;
  language: string;
  breakNotificationTime: number | null;
}

const store = await load("store.json", {
  autoSave: true,
  defaults: {
    disableAnimation: false,
    password: null,
    parentEmail: "",
    language: "ru",
    breakNotificationTime: null,
  },
});

const storage: StateStorage = {
  getItem: async (name: string): Promise<string | null> => {
    return (await store.get(name)) || null;
  },
  setItem: async (name: string, value: string): Promise<void> => {
    await store.set(name, value);
  },
  removeItem: async (name: string): Promise<void> => {
    await store.delete(name);
  },
};

export const useSettingsStore = create<{
  settings: SettingsState;
  setSettings: (settings: SettingsState) => void;
}>()(
  persist(
    (set, get) => ({
      settings: {
        disableAnimation: false,
        password: null,
        parentEmail: "",
        language: "ru",
        breakNotificationTime: null,
      },
      setSettings: (settings) =>
        set({ settings: { ...get().settings, ...settings } }),
    }),
    {
      name: "settings",
      storage: createJSONStorage(() => storage),
    },
  ),
);
