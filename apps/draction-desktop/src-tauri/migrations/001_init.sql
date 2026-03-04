PRAGMA journal_mode=WAL;
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS rules (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  enabled INTEGER NOT NULL CHECK (enabled IN (0,1)),
  order_index INTEGER NOT NULL,
  when_json TEXT NOT NULL,
  workflow_id TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS workflows (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  version INTEGER NOT NULL DEFAULT 1,
  nodes_json TEXT NOT NULL,
  edges_json TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS events (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL,
  occurred_at TEXT NOT NULL,
  source_kind TEXT,
  payload_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_files (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  original_path TEXT NOT NULL,
  inbox_path TEXT NOT NULL,
  name TEXT NOT NULL,
  ext TEXT,
  size_bytes INTEGER NOT NULL,
  mime TEXT,
  sha256 TEXT,
  is_folder INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS runs (
  id TEXT PRIMARY KEY,
  event_id TEXT NOT NULL REFERENCES events(id),
  rule_id TEXT REFERENCES rules(id),
  workflow_id TEXT NOT NULL REFERENCES workflows(id),
  status TEXT NOT NULL CHECK (status IN ('queued','running','completed','failed','cancelled')),
  started_at TEXT,
  finished_at TEXT,
  failed_node_id TEXT,
  error_code TEXT,
  error_message TEXT,
  retryable INTEGER,
  summary TEXT
);

CREATE TABLE IF NOT EXISTS run_nodes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
  node_id TEXT NOT NULL,
  node_type TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('pending','running','success','failed','skipped')),
  started_at TEXT,
  ended_at TEXT,
  error_code TEXT,
  error_message TEXT,
  output_json TEXT
);

CREATE TABLE IF NOT EXISTS artifacts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
  kind TEXT NOT NULL CHECK (kind IN ('file','link')),
  path TEXT,
  url TEXT,
  is_partial INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS undo_entries (
  id TEXT PRIMARY KEY,
  event_id TEXT NOT NULL UNIQUE REFERENCES events(id) ON DELETE CASCADE,
  mode TEXT NOT NULL CHECK (mode IN ('move','copy')),
  src_path TEXT NOT NULL,
  dst_path TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  consumed_at TEXT,
  created_at TEXT NOT NULL
);
