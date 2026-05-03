import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { Dialog, DialogContent } from "./ui/Dialog";
import { Tabs, TabsList, TabsTriggerPill } from "./ui/Tabs";
import { queryKeys, useRuns, type Run } from "../lib/query";
import { useUiStore } from "../stores/uiStore";

const STATUS_STYLES: Record<string, string> = {
  completed: "bg-success/10 text-success border border-success/30",
  running:   "bg-info/10 text-info border border-info/30",
  queued:    "bg-surface-2 text-text-subtle border border-border",
  failed:    "bg-danger/10 text-danger border border-danger/30",
  cancelled: "bg-warning/10 text-warning border border-warning/30",
};

const STATUS_DOT: Record<string, string> = {
  completed: "bg-success",
  running:   "bg-info animate-pulse",
  queued:    "bg-text-subtle",
  failed:    "bg-danger",
  cancelled: "bg-warning",
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

interface RunDetailProps {
  run: Run;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function RunDetail({ run, open, onOpenChange }: RunDetailProps) {
  const artifacts: Array<{ kind: string; path?: string; url?: string }> =
    run.artifacts_json ? JSON.parse(run.artifacts_json) : [];
  const error = run.error_json ? JSON.parse(run.error_json) : null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        title="Run Detail"
        headerSlot={
          <>
            <span className={`h-2 w-2 rounded-full ${STATUS_DOT[run.status] ?? "bg-text-subtle"}`} />
            <span
              className={`rounded-full px-2 py-0.5 text-xs font-medium ${STATUS_STYLES[run.status] ?? "bg-surface-2 text-text-subtle"}`}
            >
              {run.status}
            </span>
          </>
        }
      >
        <div className="grid grid-cols-2 gap-2 text-sm">
          {[
            ["Run ID", run.id],
            ["Event ID", run.event_id],
            ["Rule ID", run.rule_id],
            ["Workflow ID", run.workflow_id],
          ].map(([label, value]) => (
            <div key={label} className="rounded-lg bg-surface-2 p-3">
              <div className="text-xs text-text-subtle mb-1">{label}</div>
              <div className="text-text font-mono text-xs break-all">{value || "—"}</div>
            </div>
          ))}
        </div>

        <div className="grid grid-cols-2 gap-2 text-sm">
          <div className="rounded-lg bg-surface-2 p-3">
            <div className="text-xs text-text-subtle mb-1">Started</div>
            <div className="text-text text-xs">{formatTime(run.started_at)}</div>
          </div>
          {run.finished_at && (
            <div className="rounded-lg bg-surface-2 p-3">
              <div className="text-xs text-text-subtle mb-1">Finished</div>
              <div className="text-text text-xs">{formatTime(run.finished_at)}</div>
            </div>
          )}
          <div className="rounded-lg bg-surface-2 p-3">
            <div className="text-xs text-text-subtle mb-1">Duration</div>
            <div className="text-text text-xs">{duration(run.started_at, run.finished_at)}</div>
          </div>
        </div>

        {run.event?.path && (
          <div className="rounded-lg bg-surface-2 p-3">
            <div className="text-xs text-text-subtle mb-1">File</div>
            <div className="text-text font-mono text-xs break-all">{run.event.path}</div>
          </div>
        )}

        {error && (
          <div className="rounded-lg border border-danger/40 bg-danger/10 p-3">
            <p className="mb-1 text-xs font-medium text-danger">Error</p>
            <pre className="overflow-auto text-xs text-danger whitespace-pre-wrap">
              {typeof error === "string" ? error : JSON.stringify(error, null, 2)}
            </pre>
          </div>
        )}

        {artifacts.length > 0 && (
          <div>
            <p className="mb-2 text-xs font-medium text-text-subtle">Artifacts</p>
            <ul className="space-y-1">
              {artifacts.map((a, i) => (
                <li key={i} className="flex items-center gap-2 text-xs">
                  <span className="rounded bg-surface-2 px-1.5 py-0.5 text-text-subtle">{a.kind}</span>
                  <span className="truncate font-mono text-text-muted">{a.path ?? a.url ?? "—"}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}

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
      className="w-full rounded-lg border border-border bg-surface px-4 py-3 text-left transition-all hover:border-border-strong hover:bg-surface-2 group"
    >
      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-3 min-w-0">
          <span
            aria-hidden
            className={`shrink-0 h-2 w-2 rounded-full ${STATUS_DOT[run.status] ?? "bg-text-subtle"}`}
          />
          <div className="min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <span className="font-medium text-text text-sm truncate">{name}</span>
              <span
                className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${STATUS_STYLES[run.status] ?? "bg-surface-2 text-text-subtle"}`}
              >
                {run.status}
              </span>
            </div>
            <p className="mt-0.5 truncate text-xs text-text-subtle font-mono">{file}</p>
          </div>
        </div>
        <div className="flex items-center gap-3 shrink-0">
          <span className="text-xs text-text-subtle">{relativeTime(run.started_at)}</span>
          <span aria-hidden className="text-text-subtle group-hover:text-text-muted transition-colors">›</span>
        </div>
      </div>
    </button>
  );
}

type FilterTab = "all" | "completed" | "failed" | "running";

const FILTER_TABS: { key: FilterTab; label: string }[] = [
  { key: "all", label: "All" },
  { key: "completed", label: "Completed" },
  { key: "failed", label: "Failed" },
  { key: "running", label: "Running" },
];

export default function RunsPanel({ baseUrl }: { baseUrl: string }) {
  const queryClient = useQueryClient();
  const selectedRunId = useUiStore((s) => s.selectedRunId);
  const openRun = useUiStore((s) => s.openRun);
  const closeRun = useUiStore((s) => s.closeRun);
  const [filter, setFilter] = useState<FilterTab>("all");

  const runsQuery = useRuns(baseUrl);
  const runs = runsQuery.data ?? [];

  const filtered = filter === "all" ? runs : runs.filter((r) => r.status === filter);
  const selected = runs.find((r) => r.id === selectedRunId) ?? null;

  return (
    <div>
      {selected && (
        <RunDetail
          run={selected}
          open
          onOpenChange={(open) => {
            if (!open) closeRun();
          }}
        />
      )}

      <Tabs value={filter} onValueChange={(v) => setFilter(v as FilterTab)} className="mb-4">
        <div className="flex items-center justify-between gap-4">
          <TabsList className="flex gap-1">
            {FILTER_TABS.map(({ key, label }) => {
              const count =
                key === "all" ? runs.length : runs.filter((r) => r.status === key).length;
              return (
                <TabsTriggerPill key={key} value={key} badge={count}>
                  {label}
                </TabsTriggerPill>
              );
            })}
          </TabsList>
          <button
            onClick={() => queryClient.invalidateQueries({ queryKey: queryKeys.runs })}
            className="rounded px-3 py-1.5 text-xs text-text-subtle transition-colors hover:bg-surface-2 hover:text-text-muted"
          >
            Refresh
          </button>
        </div>
      </Tabs>

      {runsQuery.isError && (
        <div className="mb-3 rounded-lg border border-danger/40 bg-danger/10 p-3 text-sm text-danger">
          Failed to load runs: {String(runsQuery.error?.message ?? "unknown error")}
        </div>
      )}

      {runsQuery.isLoading && runs.length === 0 && (
        <div className="rounded-lg border border-border p-10 text-center text-text-subtle text-sm">
          Loading runs…
        </div>
      )}

      {!runsQuery.isLoading && filtered.length === 0 && !runsQuery.isError && (
        <div className="rounded-lg border border-border p-10 text-center text-text-subtle text-sm">
          No {filter !== "all" ? filter : ""} runs found.
        </div>
      )}

      {filtered.length > 0 && (
        <div className="space-y-1.5">
          {filtered.map((run) => (
            <RunRow key={run.id} run={run} onClick={() => openRun(run.id)} />
          ))}
        </div>
      )}
    </div>
  );
}
