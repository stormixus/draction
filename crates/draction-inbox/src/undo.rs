use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use anyhow::Result;

const MAX_UNDO_ENTRIES: usize = 5;
const UNDO_TTL_SECS: i64 = 10;

#[derive(Clone, Debug)]
pub struct UndoEntry {
    pub event_id: String,
    pub src_path: String,
    pub dst_path: String,
    pub is_copy: bool,
    pub created_at: DateTime<Utc>,
}

pub struct UndoStack {
    entries: VecDeque<UndoEntry>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self { entries: VecDeque::new() }
    }

    pub fn push(&mut self, entry: UndoEntry) {
        if self.entries.len() >= MAX_UNDO_ENTRIES {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn try_undo(&mut self, event_id: &str) -> Result<Option<UndoEntry>> {
        let now = Utc::now();
        if let Some(pos) = self.entries.iter().position(|e| e.event_id == event_id) {
            let entry = &self.entries[pos];
            let elapsed = (now - entry.created_at).num_seconds();
            if elapsed > UNDO_TTL_SECS {
                return Ok(None); // expired
            }
            Ok(Some(self.entries.remove(pos).unwrap()))
        } else {
            Ok(None)
        }
    }

    /// Invalidate undo for an event (e.g., workflow started).
    pub fn invalidate(&mut self, event_id: &str) {
        self.entries.retain(|e| e.event_id != event_id);
    }
}
