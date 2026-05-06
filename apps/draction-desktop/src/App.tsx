import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import RunsPanel from "./components/RunsPanel";
import RulesPanel from "./components/RulesPanel";
import { useHealth } from "./lib/query";
import { setAuthToken } from "./lib/settings";
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

function App() {
  const [baseUrl, setBaseUrl] = useState<string | null>(null);
  const [initialized, setInitialized] = useState(false);
  const activePane = useUiStore((s) => s.activePane);
  const setActivePane = useUiStore((s) => s.setActivePane);
  const health = useHealth(baseUrl);
  const apiConnected = health.isSuccess;

  const loadSettings = useSettingsStore((s) => s.loadSettings);
  const settings = useSettingsStore((s) => s.settings);
  const overlayVisible = useUiStore((s) => s.overlayVisible);
  const setOverlayVisible = useUiStore((s) => s.setOverlayVisible);

  useEffect(() => {
    invoke<number>("get_api_port")
      .then((port) => setBaseUrl(`http://127.0.0.1:${port}`))
      .catch(() => setBaseUrl("http://127.0.0.1:9400"));
  }, []);

  useEffect(() => {
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
  }, [loadSettings]);

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
        invoke("set_overlay_visible", { visible: next }).catch(() => {});
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [overlayVisible, setActivePane, setOverlayVisible]);

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
              label={item.label}
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
              label={item.label}
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
          {apiConnected ? "API connected · 127.0.0.1" : "Disconnected"}
        </div>
      </aside>

      {/* Body */}
      <main className="min-w-0 flex-1 overflow-auto p-6">
        {!baseUrl || !initialized || !settings ? (
          <div className="flex items-center justify-center py-16 text-sm text-text-subtle">
            Initializing…
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
