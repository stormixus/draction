use draction_domain::rule::{Condition, GroupMode, Op, Rule};
use serde_json::Value;
use std::collections::HashMap;

pub type EvalCtx = HashMap<String, Value>;

pub fn match_first_rule<'a>(rules: &'a [Rule], ctx: &EvalCtx) -> Option<&'a Rule> {
    rules
        .iter()
        .filter(|r| r.enabled)
        .find(|r| eval(&r.when, ctx))
}

fn eval(cond: &Condition, ctx: &EvalCtx) -> bool {
    match cond {
        Condition::Group { mode, children } => {
            let iter = children.iter().map(|c| eval(c, ctx));
            match mode {
                GroupMode::All => iter.fold(true, |a, b| a && b),
                GroupMode::Any => iter.fold(false, |a, b| a || b),
            }
        }
        Condition::Predicate { field, op, value } => {
            let Some(actual) = ctx.get(field) else { return false };
            match op {
                Op::Eq => actual == value,
                Op::In => value.as_array().is_some_and(|arr| arr.contains(actual)),
                Op::Gt => compare(actual, value).is_some_and(|o| o == std::cmp::Ordering::Greater),
                Op::Gte => compare(actual, value).is_some_and(|o| o != std::cmp::Ordering::Less),
                Op::Lt => compare(actual, value).is_some_and(|o| o == std::cmp::Ordering::Less),
                Op::Lte => compare(actual, value).is_some_and(|o| o != std::cmp::Ordering::Greater),
            }
        }
    }
}

fn compare(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a.as_f64(), b.as_f64()) {
        (Some(a), Some(b)) => a.partial_cmp(&b),
        _ => None,
    }
}
