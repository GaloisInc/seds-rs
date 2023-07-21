use seds_rs::expr::ExpressionContext;

#[test]
fn test_namespace_get() {
    let json = serde_json::json!({"a": "1", "b": {"c": "2"}});
    let namespace = ExpressionContext::from_json(&json).unwrap();

    assert_eq!(namespace.get(&["a"]).unwrap(), "1".to_string());
    assert_eq!(namespace.get(&["b", "c"]).unwrap(), "2".to_string());
}

#[test]
fn test_namespace_get_not_found() {
    let json = serde_json::json!({"a": "1", "b": {"c": "2"}});
    let namespace = ExpressionContext::from_json(&json).unwrap();

    assert!(namespace.get(&["c"]).is_err());
    assert!(namespace.get(&["b", "d"]).is_err());
}

#[test]
fn test_namespace_eval_expression() {
    let json = serde_json::json!({"a": "1", "b": "2"});
    let namespace = ExpressionContext::from_json(&json).unwrap();

    assert_eq!(
        namespace.eval_expression("${a} * ${b}").unwrap(),
        evalexpr::Value::from(2)
    );
    assert_eq!(
        namespace.eval_expression("no_vars").unwrap(),
        evalexpr::Value::from("no_vars")
    );
    assert_eq!(
        namespace.eval_expression("${a}").unwrap(),
        evalexpr::Value::from(1)
    );
}

#[test]
fn test_namespace_eval_expression_not_found() {
    let json = serde_json::json!({"a": "1", "b": "2"});
    let namespace = ExpressionContext::from_json(&json).unwrap();

    assert!(namespace.eval_expression("${c}").is_err());
}

#[test]
fn test_namespace_eval_nested_expression() {
    let json = serde_json::json!({
        "a": "1.0",
        "level1": {
            "b": "2.0",
            "level2": {
                "c": "3.0"
            }
        },
    });
    let namespace = ExpressionContext::from_json(&json).unwrap();

    assert_eq!(
        namespace
            .eval_expression("(${a} / ${level1/b}) ^ ${level1/level2/c}")
            .unwrap(),
        evalexpr::Value::from(1.0 / 8.0)
    );
}

#[test]
fn test_empty_namespace() {
    let json = serde_json::json!({});
    let namespace = ExpressionContext::from_json(&json).unwrap();
    assert!(namespace.eval_expression("${a} * ${b}").is_err());
}
