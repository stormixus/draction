// Domain types mirroring Rust structs for frontend use

export interface Rule {
  id: string;
  name: string;
  enabled: boolean;
  priority: number;
  condition: Condition;
  then_action: ThenAction;
  created_at: string;
  updated_at: string;
}

export type Condition =
  | { type: "group"; mode: "all" | "any"; children: Condition[] }
  | { type: "predicate"; field: string; op: Op; value: string };

export type Op = "eq" | "in" | "gt" | "gte" | "lt" | "lte";

export interface ThenAction {
  workflow_id: string;
  params: Record<string, string>;
}

export interface Workflow {
  id: string;
  name: string;
  nodes: WorkflowNode[];
  edges: Edge[];
  created_at: string;
  updated_at: string;
}

export interface WorkflowNode {
  id: string;
  kind: string;
  label: string;
  config: Record<string, unknown>;
}

export interface Edge {
  from: string;
  to: string;
}

export type RunStatus = "queued" | "running" | "completed" | "failed" | "cancelled";
export type NodeStatus = "pending" | "running" | "success" | "failed" | "skipped";

export interface Run {
  id: string;
  workflow_id: string;
  status: RunStatus;
  started_at: string | null;
  finished_at: string | null;
}

export interface IngestEvent {
  id: string;
  source: "drop" | "watch" | "api";
  files: IngestFile[];
  timestamp: string;
}

export interface IngestFile {
  original_name: string;
  inbox_path: string;
  size_bytes: number;
  sha256: string;
}

export interface AppState {
  pid: number;
  port: number;
  last_seen: string;
}
