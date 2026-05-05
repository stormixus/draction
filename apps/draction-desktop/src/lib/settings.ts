const API_BASE = "http://127.0.0.1:9400";

let authToken = "";

export function setAuthToken(token: string) {
  authToken = token;
}

function headers() {
  return {
    "Content-Type": "application/json",
    Authorization: `Bearer ${authToken}`,
  };
}

export interface Settings {
  launch_at_login: boolean;
  show_draky_on_launch: boolean;
  run_minimized: boolean;
  theme: string;
  accent_color: string;
  reduce_motion: boolean;
  language: string;
  date_format: string;
  inbox_location: string;
  conflict_resolution: string;
  delete_source_after_ingest: boolean;
  match_policy: string;
  concurrency: number;
  undo_window_seconds: number;
  draky_size: string;
  draky_personality: string;
  draky_always_on_top: boolean;
  openclaw_paired: boolean;
  openclaw_device_id: string;
  pairing_code: string;
  allowed_path_scopes: string[];
  dangerous_nodes_enabled: boolean;
  require_confirmation_for: string[];
  db_path: string;
  log_level: string;
  api_port: number;
}

export async function fetchSettings(): Promise<Settings> {
  const res = await fetch(`${API_BASE}/api/v1/settings`, { headers: headers() });
  if (!res.ok) throw new Error(`Failed to fetch settings: ${res.status}`);
  return res.json();
}

export async function updateSettings(partial: Partial<Settings>): Promise<Settings> {
  const res = await fetch(`${API_BASE}/api/v1/settings`, {
    method: "PUT",
    headers: headers(),
    body: JSON.stringify(partial),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err?.error?.message || `Failed to update settings: ${res.status}`);
  }
  return res.json();
}
