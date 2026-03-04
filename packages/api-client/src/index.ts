import type { Rule, Workflow, Run, IngestEvent, AppState } from "@draction/shared-types";

const DEFAULT_BASE = "http://127.0.0.1:9400";

export class DractionClient {
  private base: string;
  private token: string | null = null;

  constructor(base = DEFAULT_BASE) {
    this.base = base;
  }

  setToken(token: string) {
    this.token = token;
  }

  private headers(): HeadersInit {
    const h: Record<string, string> = { "Content-Type": "application/json" };
    if (this.token) h["Authorization"] = `Bearer ${this.token}`;
    return h;
  }

  private async request<T>(method: string, path: string, body?: unknown): Promise<T> {
    const res = await fetch(`${this.base}${path}`, {
      method,
      headers: this.headers(),
      body: body ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`${method} ${path} failed (${res.status}): ${text}`);
    }
    return res.json();
  }

  // Rules
  listRules() {
    return this.request<Rule[]>("GET", "/api/rules");
  }
  getRule(id: string) {
    return this.request<Rule>("GET", `/api/rules/${id}`);
  }
  createRule(rule: Omit<Rule, "id" | "created_at" | "updated_at">) {
    return this.request<Rule>("POST", "/api/rules", rule);
  }
  updateRule(id: string, rule: Partial<Rule>) {
    return this.request<Rule>("PUT", `/api/rules/${id}`, rule);
  }
  deleteRule(id: string) {
    return this.request<void>("DELETE", `/api/rules/${id}`);
  }

  // Workflows
  listWorkflows() {
    return this.request<Workflow[]>("GET", "/api/workflows");
  }
  getWorkflow(id: string) {
    return this.request<Workflow>("GET", `/api/workflows/${id}`);
  }

  // Runs
  listRuns(workflowId?: string) {
    const q = workflowId ? `?workflow_id=${workflowId}` : "";
    return this.request<Run[]>("GET", `/api/runs${q}`);
  }

  // Events
  listEvents() {
    return this.request<IngestEvent[]>("GET", "/api/events");
  }

  // Undo
  undo() {
    return this.request<{ undone: string }>("POST", "/api/undo");
  }

  // Health
  health() {
    return this.request<AppState>("GET", "/api/health");
  }

  // Pairing
  async pair(): Promise<string> {
    const { token } = await this.request<{ token: string }>("POST", "/api/pair");
    this.token = token;
    return token;
  }
}

export { type Rule, type Workflow, type Run, type IngestEvent, type AppState } from "@draction/shared-types";
