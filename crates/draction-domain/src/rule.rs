use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub order_index: i64,
    pub when: Condition,
    pub then: ThenAction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Condition {
    Group { mode: GroupMode, children: Vec<Condition> },
    Predicate { field: String, op: Op, value: serde_json::Value },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GroupMode {
    All,
    Any,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Eq,
    In,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThenAction {
    pub workflow_id: String,
}
