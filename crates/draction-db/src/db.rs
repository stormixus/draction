use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct RuleRow {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub order_index: i64,
    pub when_json: String,
    pub workflow_id: String,
}

#[derive(Debug, Serialize)]
pub struct WorkflowRow {
    pub id: String,
    pub name: String,
    pub nodes_json: String,
    pub edges_json: String,
}

#[derive(Debug, Serialize)]
pub struct RunRow {
    pub id: String,
    pub event_id: String,
    pub rule_id: String,
    pub workflow_id: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub error_json: Option<String>,
    pub artifacts_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EventRow {
    pub id: String,
    pub time: String,
    pub source_json: String,
    pub files_json: String,
}

pub struct DractionDb {
    conn: Mutex<Connection>,
}

impl DractionDb {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self { conn: Mutex::new(conn) };
        db.migrate()?;
        Ok(db)
    }

    fn with_conn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("lock: {e}"))?;
        f(&conn)
    }

    fn migrate(&self) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute_batch(
                "
                CREATE TABLE IF NOT EXISTS events (
                    id          TEXT PRIMARY KEY,
                    time        TEXT NOT NULL,
                    source_json TEXT NOT NULL,
                    files_json  TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS runs (
                    id             TEXT PRIMARY KEY,
                    event_id       TEXT NOT NULL,
                    rule_id        TEXT NOT NULL,
                    workflow_id    TEXT NOT NULL,
                    status         TEXT NOT NULL,
                    started_at     TEXT NOT NULL,
                    finished_at    TEXT,
                    error_json     TEXT,
                    artifacts_json TEXT
                );
                ",
            )?;
            Ok(())
        })
    }

    pub fn insert_event(
        &self,
        id: &str,
        time: &str,
        source_json: &str,
        files_json: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO events (id, time, source_json, files_json) VALUES (?1, ?2, ?3, ?4)",
                params![id, time, source_json, files_json],
            )?;
            Ok(())
        })
    }

    pub fn insert_run(
        &self,
        id: &str,
        event_id: &str,
        rule_id: &str,
        workflow_id: &str,
        status: &str,
        started_at: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO runs (id, event_id, rule_id, workflow_id, status, started_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, event_id, rule_id, workflow_id, status, started_at],
            )?;
            Ok(())
        })
    }

    pub fn update_run_status(
        &self,
        id: &str,
        status: &str,
        finished_at: Option<&str>,
        error_json: Option<&str>,
        artifacts_json: Option<&str>,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE runs SET status = ?1, finished_at = ?2, error_json = ?3, artifacts_json = ?4 \
                 WHERE id = ?5",
                params![status, finished_at, error_json, artifacts_json, id],
            )?;
            Ok(())
        })
    }

    pub fn list_runs(&self, status_filter: Option<&str>, limit: u32) -> Result<Vec<RunRow>> {
        self.with_conn(|conn| {
            let rows = if let Some(status) = status_filter {
                let mut stmt = conn.prepare(
                    "SELECT id, event_id, rule_id, workflow_id, status, started_at, \
                            finished_at, error_json, artifacts_json \
                     FROM runs WHERE status = ?1 ORDER BY started_at DESC LIMIT ?2",
                )?;
                stmt.query_map(params![status, limit], row_to_run)?
                    .collect::<rusqlite::Result<Vec<_>>>()?
            } else {
                let mut stmt = conn.prepare(
                    "SELECT id, event_id, rule_id, workflow_id, status, started_at, \
                            finished_at, error_json, artifacts_json \
                     FROM runs ORDER BY started_at DESC LIMIT ?1",
                )?;
                stmt.query_map(params![limit], row_to_run)?
                    .collect::<rusqlite::Result<Vec<_>>>()?
            };
            Ok(rows)
        })
    }

    pub fn get_run(&self, id: &str) -> Result<Option<RunRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, event_id, rule_id, workflow_id, status, started_at, \
                        finished_at, error_json, artifacts_json \
                 FROM runs WHERE id = ?1",
            )?;
            let row = stmt
                .query_row(params![id], row_to_run)
                .optional()?;
            Ok(row)
        })
    }

    pub fn list_events(&self, limit: u32) -> Result<Vec<EventRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, time, source_json, files_json FROM events ORDER BY time DESC LIMIT ?1",
            )?;
            let rows = stmt
                .query_map(params![limit], |r| {
                    Ok(EventRow {
                        id: r.get(0)?,
                        time: r.get(1)?,
                        source_json: r.get(2)?,
                        files_json: r.get(3)?,
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(rows)
        })
    }

    // ── Rules ────────────────────────────────────────────────────────────────

    pub fn list_rules(&self) -> Result<Vec<RuleRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, enabled, order_index, when_json, workflow_id \
                 FROM rules ORDER BY order_index ASC",
            )?;
            let rows = stmt.query_map([], |r| {
                Ok(RuleRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    enabled: r.get::<_, i64>(2)? != 0,
                    order_index: r.get(3)?,
                    when_json: r.get(4)?,
                    workflow_id: r.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(rows)
        })
    }

    pub fn get_rule(&self, id: &str) -> Result<Option<RuleRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, enabled, order_index, when_json, workflow_id \
                 FROM rules WHERE id = ?1",
            )?;
            Ok(stmt.query_row(params![id], |r| {
                Ok(RuleRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    enabled: r.get::<_, i64>(2)? != 0,
                    order_index: r.get(3)?,
                    when_json: r.get(4)?,
                    workflow_id: r.get(5)?,
                })
            }).optional()?)
        })
    }

    pub fn insert_rule(
        &self,
        id: &str,
        name: &str,
        enabled: bool,
        order_index: i64,
        when_json: &str,
        workflow_id: &str,
        now: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO rules (id, name, enabled, order_index, when_json, workflow_id, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id, name, enabled as i64, order_index, when_json, workflow_id, now, now],
            )?;
            Ok(())
        })
    }

    pub fn update_rule(
        &self,
        id: &str,
        name: &str,
        order_index: i64,
        when_json: &str,
        workflow_id: &str,
        now: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE rules SET name=?1, order_index=?2, when_json=?3, workflow_id=?4, updated_at=?5 WHERE id=?6",
                params![name, order_index, when_json, workflow_id, now, id],
            )?;
            Ok(())
        })
    }

    pub fn set_rule_enabled(&self, id: &str, enabled: bool, now: &str) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE rules SET enabled=?1, updated_at=?2 WHERE id=?3",
                params![enabled as i64, now, id],
            )?;
            Ok(())
        })
    }

    pub fn delete_rule(&self, id: &str) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute("DELETE FROM rules WHERE id = ?1", params![id])?;
            Ok(())
        })
    }

    // ── Workflows ─────────────────────────────────────────────────────────────

    pub fn list_workflows(&self) -> Result<Vec<WorkflowRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, nodes_json, edges_json FROM workflows ORDER BY name ASC",
            )?;
            let rows = stmt.query_map([], |r| {
                Ok(WorkflowRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    nodes_json: r.get(2)?,
                    edges_json: r.get(3)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(rows)
        })
    }

    pub fn get_workflow(&self, id: &str) -> Result<Option<WorkflowRow>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, nodes_json, edges_json FROM workflows WHERE id = ?1",
            )?;
            Ok(stmt.query_row(params![id], |r| {
                Ok(WorkflowRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    nodes_json: r.get(2)?,
                    edges_json: r.get(3)?,
                })
            }).optional()?)
        })
    }

    pub fn insert_workflow(
        &self,
        id: &str,
        name: &str,
        nodes_json: &str,
        edges_json: &str,
        now: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO workflows (id, name, version, nodes_json, edges_json, created_at, updated_at) \
                 VALUES (?1, ?2, 1, ?3, ?4, ?5, ?6)",
                params![id, name, nodes_json, edges_json, now, now],
            )?;
            Ok(())
        })
    }

    pub fn update_workflow(
        &self,
        id: &str,
        name: &str,
        nodes_json: &str,
        edges_json: &str,
        now: &str,
    ) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE workflows SET name=?1, nodes_json=?2, edges_json=?3, updated_at=?4, \
                 version = version + 1 WHERE id=?5",
                params![name, nodes_json, edges_json, now, id],
            )?;
            Ok(())
        })
    }

    /// Crash recovery: mark any runs stuck in "running" as "failed".
    pub fn mark_running_as_failed(&self) -> Result<u64> {
        self.with_conn(|conn| {
            let changed = conn.execute(
                "UPDATE runs SET status = 'failed', error_json = '\"crash recovery\"' \
                 WHERE status = 'running'",
                [],
            )?;
            Ok(changed as u64)
        })
    }
}

fn row_to_run(r: &rusqlite::Row<'_>) -> rusqlite::Result<RunRow> {
    Ok(RunRow {
        id: r.get(0)?,
        event_id: r.get(1)?,
        rule_id: r.get(2)?,
        workflow_id: r.get(3)?,
        status: r.get(4)?,
        started_at: r.get(5)?,
        finished_at: r.get(6)?,
        error_json: r.get(7)?,
        artifacts_json: r.get(8)?,
    })
}
