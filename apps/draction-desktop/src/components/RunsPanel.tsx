import { useEffect, useState, useCallback } from "react";

interface Run {
  id: string;
  event_id: string;
  rule_id: string;
  rule_name?: string;
  workflow_id: string;
  status: string;
  started_at: string;
  finished_at?: string;
  error_json?: string;
  artifacts_json?: string;
  event?: {
    path?: string;
    file_name?: string;
  };
}

const STATUS_STYLES: Record<string, string> = {
  completed: "bg-emerald-900/50 text-emerald-400 border border-emerald-800",
  running:   "bg-blue-900/50 text-blue-400 border border-blue-800",
  queued:    "bg-zinc-800 text-zinc-400 border border-zinc-700",
  failed:    "bg-red-900/50 text-red-400 border border-red-800",
  cancelled: "bg-amber-900/50 text-amber-400 border border-amber-800",
};

const STATUS_DOT: Record<string, string> = {
  completed: "bg-emerald-400",
  running:   "bg-blue-400 animate-pulse",
  queued:    "bg-zinc-500",
  failed:    "bg-red-400",
  cancelled: "bg-amber-400",
};

function relativeTime(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const s = Math.floor(diff / 1000);
  if (s < 60) return `${s}s ago`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m ago`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h ago`;
  return `${Math.floor(h / 24)}d ago`;
}

function formatTime(iso: string): string {
  try {
    return new Date(iso).toLocaleString();
  } catch {
    return iso;
  }
}

function duration(started: string, finished?: string): string {
  const s = new Date(started).getTime();
  const e = finished ? new Date(finished).getTime() : Date.now();
  const ms = e - s;
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(1)}s`;
  return `${Math.floor(ms / 60_000)}m ${Math.floor((ms % 60_000) / 1000)}s`;
}

function fileName(run: Run): string {
  if (run.event?.file_name) return run.event.file_name;
  if (run.event?.path) return run.event.path.split("/").pop() ?? run.event.path;
  return "—";
}

// --- Run Detail Modal ---
interface RunDetailProps {
  run: Run;
  onClose: () => void;
}

function RunDetail({ run, onClose }: RunDetailProps) {
  const artifacts: Array<{ kind: string; path?: string; url?: string }> =
    run.artifacts_json ? JSON.parse(run.artifacts_json) : [];
  const error = run.error_json ? JSON.parse(run.error_json) : null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-full max-w-lg rounded-xl border border-zinc-700 bg-zinc-900 shadow-2xl">
        <div className="flex items-center justify-between border-b border-zinc-800 px-6 py-4">
          <div className="flex items-center gap-3">
            <span
              className={`h-2 w-2 rounded-full ${STATUS_DOT[run.status] ?? "bg-zinc-500"}`}
            />
            <h2 className="font-semibold text-zinc-100">Run Detail</h2>
            <span
              className={`rounded-full px-2 py-0.5 text-xs font-medium ${STATUS_STYLES[run.status] ?? "bg-zinc-800 text-zinc-400"}`}
            >
              {run.status}
            </span>
          </div>
          <button
            onClick={onClose}
            className="rounded p-1 text-zinc-500 hover:bg-zinc-800 hover:text-zinc-200 transition-colors text-lg leading-none"
          >
            ×
          </button>
        </div>

        <div className="p-6 space-y-4">
          <div className="grid grid-cols-2 gap-2 text-sm">
            {[
              ["Run ID", run.id],
              ["Event ID", run.event_id],
              ["Rule ID", run.rule_id],
              ["Workflow ID", run.workflow_id],
            ].map(([label, value]) => (
              <div key={label} className="rounded-lg bg-zinc-800 p-3">
                <div className="text-xs text-zinc-500 mb-1">{label}</div>
                <div className="text-zinc-200 font-mono text-xs break-all">{value || "—"}</div>
              </div>
            ))}
          </div>

          <div className="grid grid-cols-2 gap-2 text-sm">
            <div className="rounded-lg bg-zinc-800 p-3">
              <div className="text-xs text-zinc-500 mb-1">Started</div>
              <div className="text-zinc-200 text-xs">{formatTime(run.started_at)}</div>
            </div>
            {run.finished_at && (
              <div className="rounded-lg bg-zinc-800 p-3">
                <div className="text-xs text-zinc-500 mb-1">Finished</div>
                <div className="text-zinc-200 text-xs">{formatTime(run.finished_at)}</div>
              </div>
            )}
            <div className="rounded-lg bg-zinc-800 p-3">
              <div className="text-xs text-zinc-500 mb-1">Duration</div>
              <div className="text-zinc-200 text-xs">{duration(run.started_at, run.finished_at)}</div>
            </div>
          </div>

          {run.event?.path && (
            <div className="rounded-lg bg-zinc-800 p-3">
              <div className="text-xs text-zinc-500 mb-1">File</div>
              <div className="text-zinc-200 font-mono text-xs break-all">{run.event.path}</div>
            </div>
          )}

          {error && (
            <div className="rounded-lg border border-red-800 bg-red-950/30 p-3">
              <p className="mb-1 text-xs font-medium text-red-400">Error</p>
              <pre className="overflow-auto text-xs text-red-300 whitespace-pre-wrap">
                {typeof error === "string" ? error : JSON.stringify(error, null, 2)}
              </pre>
            </div>
          )}

          {artifacts.length > 0 && (
            <div>
              <p className="mb-2 text-xs font-medium text-zinc-500">Artifacts</p>
              <ul className="space-y-1">
                {artifacts.map((a, i) => (
                  <li key={i} className="flex items-center gap-2 text-xs">
                    <span className="rounded bg-zinc-800 px-1.5 py-0.5 text-zinc-400">{a.kind}</span>
                    <span className="truncate font-mono text-zinc-300">{a.path ?? a.url ?? "—"}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

// --- Run Row ---
interface RunRowProps {
  run: Run;
  onClick: () => void;
}

function RunRow({ run, onClick }: RunRowProps) {
  const name = run.rule_name ?? run.rule_id ?? run.workflow_id;
  const file = fileName(run);

  return (
    <button
      onClick={onClick}
      className="w-full rounded-lg border border-zinc-800 bg-zinc-900 px-4 py-3 text-left transition-all hover:border-zinc-600 hover:bg-zinc-800/70 group"
    >
      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-3 min-w-0">
          <span
            className={`shrink-0 h-2 w-2 rounded-full ${STATUS_DOT[run.status] ?? "bg-zinc-500"}`}
          />
          <div className="min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <span className="font-medium text-zinc-200 text-sm truncate">{name}</span>
              <span
                className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${STATUS_STYLES[run.status] ?? "bg-zinc-800 text-zinc-400"}`}
              >
                {run.status}
              </span>
            </div>
            <p className="mt-0.5 truncate text-xs text-zinc-500 font-mono">{file}</p>
          </div>
        </div>
        <div className="flex items-center gap-3 shrink-0">
          <span className="text-xs text-zinc-600">{relativeTime(run.started_at)}</span>
          <span className="text-zinc-600 group-hover:text-zinc-400 transition-colors">›</span>
        </div>
      </div>
    </button>
  );
}

// --- Runs Panel ---
type FilterTab = "all" | "completed" | "failed" | "running";

const FILTER_TABS: { key: FilterTab; label: string }[] = [
  { key: "all", label: "All" },
  { key: "completed", label: "Completed" },
  { key: "failed", label: "Failed" },
  { key: "running", label: "Running" },
];

export default function RunsPanel({ baseUrl }: { baseUrl: string }) {
  const [runs, setRuns] = useState<Run[]>([]);
  const [loading, setLoading] = useState(true);
  const [fetchError, setFetchError] = useState<string | null>(null);
  const [selected, setSelected] = useState<Run | null>(null);
  const [filter, setFilter] = useState<FilterTab>("all");

  const fetchRuns = useCallback(async () => {
    try {
      const res = await fetch(`${baseUrl}/api/v1/runs?limit=50`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setRuns(Array.isArray(data) ? data : data.runs ?? []);
      setFetchError(null);
    } catch (e) {
      setFetchError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    fetchRuns();
    const id = setInterval(fetchRuns, 5000);
    return () => clearInterval(id);
  }, [fetchRuns]);

  const filtered =
    filter === "all" ? runs : runs.filter((r) => r.status === filter);

  return (
    <div>
      {selected && (
        <RunDetail run={selected} onClose={() => setSelected(null)} />
      )}

      <div className="mb-4 flex items-center justify-between gap-4">
        <div className="flex gap-1">
          {FILTER_TABS.map(({ key, label }) => {
            const count =
              key === "all" ? runs.length : runs.filter((r) => r.status === key).length;
            return (
              <button
                key={key}
                onClick={() => setFilter(key)}
                className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
                  filter === key
                    ? "bg-zinc-700 text-zinc-100"
                    : "text-zinc-500 hover:bg-zinc-800 hover:text-zinc-300"
                }`}
              >
                {label}
                <span
                  className={`rounded-full px-1.5 py-0.5 text-xs ${
                    filter === key ? "bg-zinc-600 text-zinc-300" : "bg-zinc-800 text-zinc-500"
                  }`}
                >
                  {count}
                </span>
              </button>
            );
          })}
        </div>
        <button
          onClick={fetchRuns}
          className="rounded px-3 py-1.5 text-xs text-zinc-500 hover:bg-zinc-800 hover:text-zinc-300 transition-colors"
        >
          Refresh
        </button>
      </div>

      {fetchError && (
        <div className="mb-3 rounded-lg border border-red-800 bg-red-950/30 p-3 text-sm text-red-400">
          Failed to load runs: {fetchError}
        </div>
      )}

      {loading && runs.length === 0 && (
        <div className="rounded-lg border border-zinc-800 p-10 text-center text-zinc-600 text-sm">
          Loading runs…
        </div>
      )}

      {!loading && filtered.length === 0 && !fetchError && (
        <div className="rounded-lg border border-zinc-800 p-10 text-center text-zinc-500 text-sm">
          No {filter !== "all" ? filter : ""} runs found.
        </div>
      )}

      {filtered.length > 0 && (
        <div className="space-y-1.5">
          {filtered.map((run) => (
            <RunRow key={run.id} run={run} onClick={() => setSelected(run)} />
          ))}
        </div>
      )}
    </div>
  );
}
