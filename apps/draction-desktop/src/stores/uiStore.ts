import { create } from "zustand";

type Theme = "system" | "light" | "dark";
type DashboardTab = "runs" | "rules";

interface UiState {
  theme: Theme;
  activeTab: DashboardTab;
  selectedRunId: string | null;
  overlayVisible: boolean;
  lastUndoRunId: string | null;

  setTheme: (theme: Theme) => void;
  setActiveTab: (tab: DashboardTab) => void;
  openRun: (runId: string) => void;
  closeRun: () => void;
  setOverlayVisible: (visible: boolean) => void;
  setLastUndoRunId: (runId: string | null) => void;
}

export const useUiStore = create<UiState>((set) => ({
  theme: "dark",
  activeTab: "runs",
  selectedRunId: null,
  overlayVisible: true,
  lastUndoRunId: null,

  setTheme: (theme) => set({ theme }),
  setActiveTab: (activeTab) => set({ activeTab }),
  openRun: (runId) => set({ activeTab: "runs", selectedRunId: runId }),
  closeRun: () => set({ selectedRunId: null }),
  setOverlayVisible: (overlayVisible) => set({ overlayVisible }),
  setLastUndoRunId: (lastUndoRunId) => set({ lastUndoRunId }),
}));

/**
 * Reflect the current theme onto `<html data-theme="...">`. Call this once at app boot.
 * Phase A keeps the dark theme as default; Phase C will toggle this to "light".
 */
export function applyThemeToDocument(theme: Theme) {
  const resolved =
    theme === "system"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : theme;
  document.documentElement.dataset.theme = resolved;
}
