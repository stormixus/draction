import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { authHeaders } from "./settings";

export const queryKeys = {
  runs: ["runs"] as const,
  rules: ["rules"] as const,
  health: ["health"] as const,
};

async function fetchJson<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(url, {
    ...init,
    headers: authHeaders(init?.headers),
  });
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`HTTP ${res.status}: ${text || res.statusText}`);
  }
  return res.json() as Promise<T>;
}

function asArray<T>(payload: unknown, key: string): T[] {
  if (Array.isArray(payload)) return payload as T[];
  if (payload && typeof payload === "object" && key in payload) {
    const inner = (payload as Record<string, unknown>)[key];
    return Array.isArray(inner) ? (inner as T[]) : [];
  }
  return [];
}

export interface Run {
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
  event?: { path?: string; file_name?: string };
}

export interface Rule {
  id: string;
  name: string;
  enabled: boolean;
  description?: string;
}

export function useRuns(baseUrl: string | null) {
  return useQuery({
    queryKey: [...queryKeys.runs, baseUrl] as const,
    enabled: !!baseUrl,
    refetchInterval: 5000,
    queryFn: async () => {
      const data = await fetchJson<unknown>(`${baseUrl}/api/v1/runs?limit=50`);
      return asArray<Run>(data, "runs");
    },
  });
}

export function useRules(baseUrl: string | null) {
  return useQuery({
    queryKey: [...queryKeys.rules, baseUrl] as const,
    enabled: !!baseUrl,
    refetchInterval: 5000,
    queryFn: async () => {
      const data = await fetchJson<unknown>(`${baseUrl}/api/v1/rules`);
      return asArray<Rule>(data, "rules");
    },
  });
}

export function useHealth(baseUrl: string | null) {
  return useQuery({
    queryKey: [...queryKeys.health, baseUrl] as const,
    enabled: !!baseUrl,
    refetchInterval: 5000,
    retry: false,
    queryFn: async () => {
      // We just need to know if the server answers 2xx; treat any 2xx body as healthy.
      const res = await fetch(`${baseUrl}/api/v1/runs?limit=1`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      return { ok: true } as const;
    },
  });
}

export function useToggleRule(baseUrl: string | null) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: async ({ id, enabled }: { id: string; enabled: boolean }) => {
      if (!baseUrl) throw new Error("API base URL not ready");
      await fetchJson<unknown>(`${baseUrl}/api/v1/rules/${id}/enabled`, {
        method: "PATCH",
        body: JSON.stringify({ enabled }),
      });
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: queryKeys.rules });
    },
  });
}
