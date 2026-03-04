import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import RunsPanel from "./components/RunsPanel";
import RulesPanel from "./components/RulesPanel";

function App() {
  const [activeTab, setActiveTab] = useState<"runs" | "rules">("runs");
  const [baseUrl, setBaseUrl] = useState<string | null>(null);
  const [apiConnected, setApiConnected] = useState(false);

  useEffect(() => {
    invoke<number>("get_api_port")
      .then((port) => setBaseUrl(`http://127.0.0.1:${port}`))
      .catch(() => setBaseUrl("http://127.0.0.1:9400"));
  }, []);

  useEffect(() => {
    if (!baseUrl) return;
    const check = async () => {
      try {
        const res = await fetch(`${baseUrl}/api/v1/runs?limit=1`);
        setApiConnected(res.ok);
      } catch {
        setApiConnected(false);
      }
    };
    check();
    const id = setInterval(check, 5000);
    return () => clearInterval(id);
  }, [baseUrl]);

  const tabs = [
    { key: "runs" as const, label: "Runs" },
    { key: "rules" as const, label: "Rules" },
  ];

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
      <header className="flex items-center justify-between border-b border-zinc-800 px-6 py-4 shrink-0">
        <h1 className="text-lg font-bold tracking-tight">Draction</h1>
        <div className="flex items-center gap-2">
          <span
            className={`h-2 w-2 rounded-full ${
              apiConnected ? "bg-emerald-400" : "bg-red-500"
            }`}
          />
          <span className="text-xs text-zinc-500">
            {apiConnected ? "Connected" : "Disconnected"}
          </span>
        </div>
      </header>

      <nav className="flex gap-1 border-b border-zinc-800 px-6 shrink-0">
        {tabs.map(({ key, label }) => (
          <button
            key={key}
            onClick={() => setActiveTab(key)}
            className={`px-4 py-3 text-sm font-medium transition-colors ${
              activeTab === key
                ? "border-b-2 border-emerald-500 text-emerald-400"
                : "text-zinc-500 hover:text-zinc-300"
            }`}
          >
            {label}
          </button>
        ))}
      </nav>

      <main className="flex-1 overflow-auto p-6">
        {!baseUrl ? (
          <div className="flex items-center justify-center py-16 text-zinc-600 text-sm">
            Initializing…
          </div>
        ) : activeTab === "runs" ? (
          <RunsPanel baseUrl={baseUrl} />
        ) : (
          <RulesPanel baseUrl={baseUrl} />
        )}
      </main>
    </div>
  );
}

export default App;
