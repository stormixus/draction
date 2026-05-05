import { create } from "zustand";
import type { Settings } from "../lib/settings";
import { fetchSettings, updateSettings } from "../lib/settings";

interface SettingsState {
  settings: Settings | null;
  loading: boolean;
  error: string | null;

  loadSettings: () => Promise<void>;
  updateSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  loading: false,
  error: null,

  loadSettings: async () => {
    set({ loading: true, error: null });
    try {
      const settings = await fetchSettings();
      set({ settings, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  updateSetting: async <K extends keyof Settings>(key: K, value: Settings[K]) => {
    const current = get().settings;
    if (!current) return;

    // Optimistic update
    set({ settings: { ...current, [key]: value } });

    try {
      const updated = await updateSettings({ [key]: value });
      set({ settings: updated });
    } catch (err) {
      // Revert on failure
      set({ settings: current, error: String(err) });
    }
  },
}));
