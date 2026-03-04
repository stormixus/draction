use uuid::Uuid;

pub fn new_event_id() -> String {
    format!("evt_{}", Uuid::new_v4().simple())
}

pub fn new_run_id() -> String {
    format!("run_{}", Uuid::new_v4().simple())
}

pub fn new_rule_id() -> String {
    format!("rule_{}", Uuid::new_v4().simple())
}

pub fn new_workflow_id() -> String {
    format!("wf_{}", Uuid::new_v4().simple())
}
