import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import RunsPanel from "./components/RunsPanel";
import RulesPanel from "./components/RulesPanel";
import { Tabs, TabsList, TabsContent, TabsTriggerUnderline } from "./components/ui/Tabs";
import { useHealth } from "./lib/query";
import { useUiStore } from "./stores/uiStore";

const TABS = [
  { key: "runs" as const, label: "Runs" },
  { key: "rules" as const, label: "Rules" },
];

function App() {
  const [baseUrl, setBaseUrl] = useState<string | null>(null);
  const activeTab = useUiStore((s) => s.activeTab);
  const setActiveTab = useUiStore((s) => s.setActiveTab);
  const health = useHealth(baseUrl);
  const apiConnected = health.isSuccess;

  useEffect(() => {
    invoke<number>("get_api_port")
      .then((port) => setBaseUrl(`http://127.0.0.1:${port}`))
      .catch(() => setBaseUrl("http://127.0.0.1:9400"));
  }, []);

  return (
    <div className="min-h-screen bg-bg text-text flex flex-col">
      <header className="flex items-center justify-between border-b border-border px-6 py-4 shrink-0">
        <h1 className="text-lg font-bold tracking-tight">Draction</h1>
        <div className="flex items-center gap-2">
          <span
            aria-hidden
            className={`h-2 w-2 rounded-full ${apiConnected ? "bg-success" : "bg-danger"}`}
          />
          <span className="text-xs text-text-subtle">
            {apiConnected ? "Connected" : "Disconnected"}
          </span>
        </div>
      </header>

      <Tabs
        value={activeTab}
        onValueChange={(v) => setActiveTab(v as typeof activeTab)}
        className="flex flex-1 flex-col"
      >
        <TabsList className="flex gap-1 border-b border-border px-6 shrink-0">
          {TABS.map(({ key, label }) => (
            <TabsTriggerUnderline key={key} value={key}>
              {label}
            </TabsTriggerUnderline>
          ))}
        </TabsList>

        <main className="flex-1 overflow-auto p-6">
          {!baseUrl ? (
            <div className="flex items-center justify-center py-16 text-text-subtle text-sm">
              Initializing…
            </div>
          ) : (
            <>
              <TabsContent value="runs"><RunsPanel baseUrl={baseUrl} /></TabsContent>
              <TabsContent value="rules"><RulesPanel baseUrl={baseUrl} /></TabsContent>
            </>
          )}
        </main>
      </Tabs>
    </div>
  );
}

export default App;
