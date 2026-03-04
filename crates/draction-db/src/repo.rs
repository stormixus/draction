use crate::Database;
use std::sync::Arc;

pub struct Repository {
    pub db: Arc<Database>,
}

impl Repository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}
