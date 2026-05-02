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
            let mut iter = children.iter().map(|c| eval(c, ctx));
            match mode {
                GroupMode::All => iter.all(|x| x),
                GroupMode::Any => iter.any(|x| x),
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

#[cfg(test)]
mod tests {
    use super::*;
    use draction_domain::rule::{Condition, GroupMode, Op, Rule, ThenAction};
    use serde_json::json;

    fn rule(id: &str, enabled: bool, when: Condition) -> Rule {
        Rule {
            id: id.to_string(),
            name: id.to_string(),
            enabled,
            order_index: 0,
            when,
            then: ThenAction { workflow_id: "wf".into() },
        }
    }

    fn predicate(field: &str, op: Op, value: Value) -> Condition {
        Condition::Predicate { field: field.into(), op, value }
    }

    fn ctx_for(name: &str, ext: &str, size: u64) -> EvalCtx {
        let mut c = EvalCtx::new();
        c.insert("name".into(), json!(name));
        c.insert("ext".into(), json!(ext));
        c.insert("size_bytes".into(), json!(size));
        c
    }

    #[test]
    fn match_first_rule_returns_first_enabled_match() {
        let rules = vec![
            rule("r1", true, predicate("ext", Op::Eq, json!("png"))),
            rule("r2", true, predicate("ext", Op::Eq, json!("jpg"))),
        ];
        let ctx = ctx_for("a.jpg", "jpg", 100);
        assert_eq!(match_first_rule(&rules, &ctx).unwrap().id, "r2");
    }

    #[test]
    fn match_first_rule_skips_disabled_even_when_matching() {
        let rules = vec![
            rule("r1", false, predicate("ext", Op::Eq, json!("jpg"))),
            rule("r2", true, predicate("ext", Op::Eq, json!("jpg"))),
        ];
        let ctx = ctx_for("a.jpg", "jpg", 100);
        assert_eq!(match_first_rule(&rules, &ctx).unwrap().id, "r2");
    }

    #[test]
    fn match_first_rule_returns_none_when_nothing_matches() {
        let rules = vec![rule("r1", true, predicate("ext", Op::Eq, json!("png")))];
        let ctx = ctx_for("a.jpg", "jpg", 100);
        assert!(match_first_rule(&rules, &ctx).is_none());
    }

    #[test]
    fn op_eq_matches_string_value() {
        let cond = predicate("ext", Op::Eq, json!("png"));
        assert!(eval(&cond, &ctx_for("a.png", "png", 1)));
        assert!(!eval(&cond, &ctx_for("a.jpg", "jpg", 1)));
    }

    #[test]
    fn op_in_checks_array_membership() {
        let cond = predicate("ext", Op::In, json!(["mp4", "mov", "avi"]));
        assert!(eval(&cond, &ctx_for("v", "mov", 1)));
        assert!(!eval(&cond, &ctx_for("v", "wmv", 1)));
    }

    #[test]
    fn op_in_returns_false_when_value_is_not_array() {
        let cond = predicate("ext", Op::In, json!("mp4"));
        assert!(!eval(&cond, &ctx_for("v", "mp4", 1)));
    }

    #[test]
    fn op_gt_gte_lt_lte_compare_numerically() {
        let gt = predicate("size_bytes", Op::Gt, json!(1000));
        let gte = predicate("size_bytes", Op::Gte, json!(1000));
        let lt = predicate("size_bytes", Op::Lt, json!(1000));
        let lte = predicate("size_bytes", Op::Lte, json!(1000));

        let small = ctx_for("f", "x", 500);
        let exact = ctx_for("f", "x", 1000);
        let big = ctx_for("f", "x", 2000);

        assert!(!eval(&gt, &small));
        assert!(!eval(&gt, &exact));
        assert!(eval(&gt, &big));

        assert!(!eval(&gte, &small));
        assert!(eval(&gte, &exact));
        assert!(eval(&gte, &big));

        assert!(eval(&lt, &small));
        assert!(!eval(&lt, &exact));
        assert!(!eval(&lt, &big));

        assert!(eval(&lte, &small));
        assert!(eval(&lte, &exact));
        assert!(!eval(&lte, &big));
    }

    #[test]
    fn comparison_against_non_numeric_returns_false() {
        let cond = predicate("ext", Op::Gt, json!("abc"));
        assert!(!eval(&cond, &ctx_for("a", "png", 1)));
    }

    #[test]
    fn missing_field_returns_false_for_any_op() {
        let ctx = EvalCtx::new();
        assert!(!eval(&predicate("ext", Op::Eq, json!("x")), &ctx));
        assert!(!eval(&predicate("ext", Op::In, json!(["x"])), &ctx));
        assert!(!eval(&predicate("size_bytes", Op::Gt, json!(0)), &ctx));
    }

    #[test]
    fn group_all_requires_every_child_true() {
        let cond = Condition::Group {
            mode: GroupMode::All,
            children: vec![
                predicate("ext", Op::Eq, json!("png")),
                predicate("size_bytes", Op::Gt, json!(100)),
            ],
        };
        assert!(eval(&cond, &ctx_for("a.png", "png", 200)));
        assert!(!eval(&cond, &ctx_for("a.png", "png", 50)));
        assert!(!eval(&cond, &ctx_for("a.jpg", "jpg", 200)));
    }

    #[test]
    fn group_any_requires_at_least_one_child_true() {
        let cond = Condition::Group {
            mode: GroupMode::Any,
            children: vec![
                predicate("ext", Op::Eq, json!("png")),
                predicate("size_bytes", Op::Gt, json!(1_000_000)),
            ],
        };
        assert!(eval(&cond, &ctx_for("a.png", "png", 50)));
        assert!(eval(&cond, &ctx_for("a.jpg", "jpg", 2_000_000)));
        assert!(!eval(&cond, &ctx_for("a.jpg", "jpg", 50)));
    }

    #[test]
    fn group_all_with_no_children_evaluates_true() {
        let cond = Condition::Group { mode: GroupMode::All, children: vec![] };
        assert!(eval(&cond, &EvalCtx::new()));
    }

    #[test]
    fn group_any_with_no_children_evaluates_false() {
        let cond = Condition::Group { mode: GroupMode::Any, children: vec![] };
        assert!(!eval(&cond, &EvalCtx::new()));
    }

    #[test]
    fn nested_groups_evaluate_correctly() {
        let cond = Condition::Group {
            mode: GroupMode::All,
            children: vec![
                predicate("ext", Op::In, json!(["mp4", "mov"])),
                Condition::Group {
                    mode: GroupMode::Any,
                    children: vec![
                        predicate("size_bytes", Op::Gt, json!(1_000_000)),
                        predicate("name", Op::Eq, json!("important.mov")),
                    ],
                },
            ],
        };
        assert!(eval(&cond, &ctx_for("important.mov", "mov", 100)));
        assert!(eval(&cond, &ctx_for("video.mp4", "mp4", 5_000_000)));
        assert!(!eval(&cond, &ctx_for("small.mp4", "mp4", 100)));
        assert!(!eval(&cond, &ctx_for("important.mov", "txt", 100)));
    }
}
