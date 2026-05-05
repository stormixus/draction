import { create } from "zustand";

type Theme = "system" | "light" | "dark";

type AppPane =
  | "runs"
  | "rules"
  | "general"
  | "inbox"
  | "rules-behavior"
  | "draky"
  | "connections"
  | "safety"
  | "advanced"
  | "about";

interface UiState {
  theme: Theme;
  activePane: AppPane;
  selectedRunId: string | null;
  overlayVisible: boolean;
  lastUndoRunId: string | null;

  setTheme: (theme: Theme) => void;
  setActivePane: (pane: AppPane) => void;
  openRun: (runId: string) => void;
  closeRun: () => void;
  setOverlayVisible: (visible: boolean) => void;
  setLastUndoRunId: (runId: string | null) => void;
}

export const useUiStore = create<UiState>((set) => ({
  theme: "dark",
  activePane: "runs",
  selectedRunId: null,
  overlayVisible: true,
  lastUndoRunId: null,

  setTheme: (theme) => set({ theme }),
  setActivePane: (activePane) => set({ activePane }),
  openRun: (runId) => set({ activePane: "runs", selectedRunId: runId }),
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
