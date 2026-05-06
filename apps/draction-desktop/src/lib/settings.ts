let apiBase = "http://127.0.0.1:9400";

let authToken = "";

export function setApiBaseUrl(baseUrl: string) {
  apiBase = baseUrl;
}

export function setAuthToken(token: string) {
  authToken = token;
}

export function authHeaders(extra?: HeadersInit) {
  return {
    "Content-Type": "application/json",
    Authorization: `Bearer ${authToken}`,
    ...extra,
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
  date_subfolders: boolean;
  inbox_size_limit_gb: number;
  auto_archive_days: number;
  undo_history_depth: number;
  match_policy: string;
  concurrency: number;
  undo_window_seconds: number;
  unmatched_files_policy: string;
  node_failure_policy: string;
  auto_retry: boolean;
  keep_partial_artifacts: boolean;
  toast_on_success: boolean;
  toast_on_failure: boolean;
  notification_sound: string;
  draky_size: string;
  draky_personality: string;
  draky_always_on_top: boolean;
  draky_overlay_visible: boolean;
  draky_snap_to_corner: boolean;
  draky_burp_on_success: boolean;
  draky_idle_behaviors: boolean;
  draky_file_type_munch: boolean;
  openclaw_paired: boolean;
  openclaw_device_id: string;
  pairing_code: string;
  api_bind_address: string;
  openclaw_auto_suggest_rules: boolean;
  openclaw_send_file_metadata: boolean;
  allowed_path_scopes: string[];
  dangerous_nodes_enabled: boolean;
  allowed_dangerous_nodes: string[];
  require_confirmation_for: string[];
  db_path: string;
  rules_path: string;
  workflows_path: string;
  log_level: string;
  log_retention_days: number;
  api_port: number;
  max_file_size_mb: number;
}

export async function fetchSettings(): Promise<Settings> {
  const res = await fetch(`${apiBase}/api/v1/settings`, { headers: authHeaders() });
  if (!res.ok) throw new Error(`Failed to fetch settings: ${res.status}`);
  return res.json();
}

export async function updateSettings(partial: Partial<Settings>): Promise<Settings> {
  const res = await fetch(`${apiBase}/api/v1/settings`, {
    method: "PUT",
    headers: authHeaders(),
    body: JSON.stringify(partial),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err?.error?.message || `Failed to update settings: ${res.status}`);
  }
  return res.json();
}
