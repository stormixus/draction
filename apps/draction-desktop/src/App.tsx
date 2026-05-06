import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import RunsPanel from "./components/RunsPanel";
import RulesPanel from "./components/RulesPanel";
import { useHealth } from "./lib/query";
import { setApiBaseUrl, setAuthToken } from "./lib/settings";
import { useI18n } from "./lib/i18n";
import { useUiStore } from "./stores/uiStore";
import { useSettingsStore } from "./stores/settingsStore";
import { NavItem } from "./components/settings/NavItem";
import {
  GeneralIcon,
  InboxIcon,
  RulesIcon,
  DrakyIcon,
  LinkIcon,
  ShieldIcon,
  BeakerIcon,
  AboutIcon,
} from "./components/settings/icons";
import { GeneralPane } from "./components/settings/panes/GeneralPane";
import { InboxPane } from "./components/settings/panes/InboxPane";
import { RulesBehaviorPane } from "./components/settings/panes/RulesBehaviorPane";
import { DrakyPane } from "./components/settings/panes/DrakyPane";
import { ConnectionsPane } from "./components/settings/panes/ConnectionsPane";
import { SafetyPane } from "./components/settings/panes/SafetyPane";
import { AdvancedPane } from "./components/settings/panes/AdvancedPane";
import { AboutPane } from "./components/settings/panes/AboutPane";

const DASHBOARD_ITEMS = [
  { key: "runs" as const, icon: <InboxIcon />, label: "Runs" },
  { key: "rules" as const, icon: <RulesIcon />, label: "Rules" },
];

const SETTINGS_ITEMS = [
  { key: "general" as const, icon: <GeneralIcon />, label: "General" },
  { key: "inbox" as const, icon: <InboxIcon />, label: "Inbox" },
  { key: "rules-behavior" as const, icon: <RulesIcon />, label: "Rules behavior" },
  { key: "draky" as const, icon: <DrakyIcon />, label: "Draky", draky: true },
  { key: "connections" as const, icon: <LinkIcon />, label: "Connections", badge: "OpenClaw" },
  { key: "safety" as const, icon: <ShieldIcon />, label: "Safety" },
  { key: "advanced" as const, icon: <BeakerIcon />, label: "Advanced" },
  { key: "about" as const, icon: <AboutIcon />, label: "About" },
];

declare global {
  interface Window {
    __dractionOpenSettings?: () => void;
  }
}

function App() {
  const t = useI18n();
  const [baseUrl, setBaseUrl] = useState<string | null>(null);
  const [initialized, setInitialized] = useState(false);
  const activePane = useUiStore((s) => s.activePane);
  const setActivePane = useUiStore((s) => s.setActivePane);
  const health = useHealth(baseUrl);
  const apiConnected = health.isSuccess;

  const loadSettings = useSettingsStore((s) => s.loadSettings);
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);
  const overlayVisible = useUiStore((s) => s.overlayVisible);
  const setOverlayVisible = useUiStore((s) => s.setOverlayVisible);

  useEffect(() => {
    invoke<number>("get_api_port")
      .then((port) => {
        const nextBaseUrl = `http://127.0.0.1:${port}`;
        setApiBaseUrl(nextBaseUrl);
        setBaseUrl(nextBaseUrl);
      })
      .catch(() => {
        const fallbackBaseUrl = "http://127.0.0.1:9400";
        setApiBaseUrl(fallbackBaseUrl);
        setBaseUrl(fallbackBaseUrl);
      });
  }, []);

  useEffect(() => {
    if (!baseUrl) return;
    setInitialized(false);

    // After the API port is known, grab the auth token and load settings
    invoke<string>("get_auth_token")
      .then((token) => {
        setAuthToken(token);
        return loadSettings();
      })
      .catch(() => {
        // Fallback: try loading settings without auth (may work in dev)
        return loadSettings();
      })
      .finally(() => setInitialized(true));
  }, [baseUrl, loadSettings]);

  useEffect(() => {
    if (!settings) return;
    const resolvedTheme =
      settings.theme === "system"
        ? window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "dark"
          : "light"
        : settings.theme;
    document.documentElement.dataset.theme = resolvedTheme;
    document.documentElement.style.setProperty("--ui-accent", settings.accent_color);
    document.documentElement.style.setProperty("--ui-focus", settings.accent_color);
    setOverlayVisible(settings.draky_overlay_visible);
    invoke("set_overlay_visible", { visible: settings.draky_overlay_visible }).catch(() => {});
  }, [settings, setOverlayVisible]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const openSettings = () => {
      setActivePane("general");
      window.focus();
    };

    window.__dractionOpenSettings = openSettings;

    listen("open-settings", openSettings).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
      if (window.__dractionOpenSettings === openSettings) {
        delete window.__dractionOpenSettings;
      }
    };
  }, [setActivePane]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const isMod = e.metaKey || e.ctrlKey;
      if (isMod && e.key === ",") {
        e.preventDefault();
        setActivePane("general");
      }
      if (isMod && e.shiftKey && (e.key === "D" || e.key === "d")) {
        e.preventDefault();
        const next = !overlayVisible;
        setOverlayVisible(next);
        updateSetting("draky_overlay_visible", next);
        invoke("set_overlay_visible", { visible: next }).catch(() => {});
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [overlayVisible, setActivePane, setOverlayVisible, updateSetting]);

  return (
    <div className="flex min-h-screen bg-bg text-text">
      {/* Sidebar */}
      <aside className="flex w-[196px] shrink-0 flex-col border-r border-border bg-sidebar py-3.5 px-2.5">
        {/* Brand */}
        <div className="mb-2.5 flex items-center gap-2 border-b border-border pb-3.5 pt-1">
          <div
            className="shrink-0 rounded-md"
            style={{
              width: 28,
              height: 28,
              background:
                "radial-gradient(circle at 40% 40%, #2dd4bf, #0f766e)",
            }}
          />
          <div>
            <div className="text-[13px] font-semibold">Draction</div>
            <div className="font-mono text-[10.5px] text-text-subtle">v0.1.4 · 9400</div>
          </div>
        </div>

        {/* Dashboard group */}
        <div className="flex flex-col gap-0.5">
          {DASHBOARD_ITEMS.map((item) => (
            <NavItem
              key={item.key}
              icon={item.icon}
              label={t(item.label)}
              active={activePane === item.key}
              onClick={() => setActivePane(item.key)}
            />
          ))}
        </div>

        <div className="my-2 border-b border-border" />

        {/* Settings group */}
        <div className="flex flex-col gap-0.5">
          {SETTINGS_ITEMS.map((item) => (
            <NavItem
              key={item.key}
              icon={item.icon}
              label={t(item.label)}
              badge={item.badge}
              active={activePane === item.key}
              draky={item.draky}
              onClick={() => setActivePane(item.key)}
            />
          ))}
        </div>

        <div className="flex-1" />

        {/* API status */}
        <div className="flex items-center gap-1.5 px-2.5 pt-2 text-[11px] text-text-subtle">
          <span
            className="h-1.5 w-1.5 rounded-full"
            style={{
              background: apiConnected ? "var(--color-accent)" : "var(--color-danger)",
              boxShadow: apiConnected ? "0 0 6px var(--color-accent)" : "none",
            }}
          />
          {apiConnected ? t("API connected · 127.0.0.1") : t("Disconnected")}
        </div>
      </aside>

      {/* Body */}
      <main className="min-w-0 flex-1 overflow-auto p-6">
        {!baseUrl || !initialized || !settings ? (
          <div className="flex items-center justify-center py-16 text-sm text-text-subtle">
            {t("Initializing…")}
          </div>
        ) : (
          <>
            {activePane === "runs" && <RunsPanel baseUrl={baseUrl} />}
            {activePane === "rules" && <RulesPanel baseUrl={baseUrl} />}
            {activePane === "general" && <GeneralPane />}
            {activePane === "inbox" && <InboxPane />}
            {activePane === "rules-behavior" && <RulesBehaviorPane />}
            {activePane === "draky" && <DrakyPane />}
            {activePane === "connections" && <ConnectionsPane />}
            {activePane === "safety" && <SafetyPane />}
            {activePane === "advanced" && <AdvancedPane />}
            {activePane === "about" && <AboutPane />}
          </>
        )}
      </main>
    </div>
  );
}

export default App;
