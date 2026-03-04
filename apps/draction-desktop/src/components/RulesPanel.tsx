import { useEffect, useState, useCallback } from "react";

interface Rule {
  id: string;
  name: string;
  enabled: boolean;
  description?: string;
}

export default function RulesPanel({ baseUrl }: { baseUrl: string }) {
  const [rules, setRules] = useState<Rule[]>([]);
  const [loading, setLoading] = useState(true);
  const [fetchError, setFetchError] = useState<string | null>(null);
  const [toggling, setToggling] = useState<string | null>(null);

  const fetchRules = useCallback(async () => {
    try {
      const res = await fetch(`${baseUrl}/api/v1/rules`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setRules(Array.isArray(data) ? data : data.rules ?? []);
      setFetchError(null);
    } catch (e) {
      setFetchError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    fetchRules();
    const id = setInterval(fetchRules, 5000);
    return () => clearInterval(id);
  }, [fetchRules]);

  const toggleRule = async (rule: Rule) => {
    setToggling(rule.id);
    try {
      await fetch(`${baseUrl}/api/v1/rules/${rule.id}/enabled`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ enabled: !rule.enabled }),
      });
      await fetchRules();
    } catch {
      // silently fail — fetchRules will show the error
    } finally {
      setToggling(null);
    }
  };

  if (loading && rules.length === 0) {
    return (
      <div className="rounded-lg border border-zinc-800 p-10 text-center text-zinc-600 text-sm">
        Loading rules…
      </div>
    );
  }

  return (
    <div>
      {fetchError && (
        <div className="mb-3 rounded-lg border border-red-800 bg-red-950/30 p-3 text-sm text-red-400">
          Failed to load rules: {fetchError}
        </div>
      )}

      {!loading && rules.length === 0 && !fetchError && (
        <div className="rounded-lg border border-zinc-800 p-10 text-center text-zinc-500 text-sm">
          No rules configured.
        </div>
      )}

      {rules.length > 0 && (
        <div className="space-y-2">
          {rules.map((rule) => (
            <div
              key={rule.id}
              className="flex items-center justify-between rounded-lg border border-zinc-800 bg-zinc-900 px-4 py-3 hover:border-zinc-700 transition-colors"
            >
              <div className="min-w-0 mr-4">
                <div className="font-medium text-zinc-200 text-sm">{rule.name}</div>
                {rule.description && (
                  <div className="text-xs text-zinc-500 mt-0.5">{rule.description}</div>
                )}
                <div className="text-xs text-zinc-600 font-mono mt-1 truncate">{rule.id}</div>
              </div>
              <button
                onClick={() => toggleRule(rule)}
                disabled={toggling === rule.id}
                aria-label={rule.enabled ? "Disable rule" : "Enable rule"}
                className={`relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors focus:outline-none ${
                  rule.enabled ? "bg-emerald-600" : "bg-zinc-700"
                } ${toggling === rule.id ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
              >
                <span
                  className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow transition-transform ${
                    rule.enabled ? "translate-x-4" : "translate-x-1"
                  }`}
                />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
