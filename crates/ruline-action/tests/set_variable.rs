#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_action::Action;
use ruline_context::Context;
use serde_json::json;

#[test]
fn test_set_variable() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "set_variable",
        "variable": "key",
        "value": {
            "type": "value",
            "value": "value"
        }
    });

    let action = Action::try_from(definition).unwrap();
    action.process(&context).unwrap();

    assert_eq!(context.get_variable("key"), Some(json!("value")));
}

#[test]
fn test_set_variable_already_exists() {
    let variables = DashMap::new();
    variables.insert("key".to_string(), json!("value"));

    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "set_variable",
        "variable": "key",
        "value": {
            "type": "value",
            "value": "new_value"
        }
    });

    let action = Action::try_from(definition).unwrap();
    action.process(&context).unwrap();

    assert_eq!(context.get_variable("key"), Some(json!("new_value")));
}

#[test]
fn test_set_variable_invalid_variable_type() {
    let definition = json!({
        "type": "set_variable",
        "variable": 300,
        "value": {
            "type": "value",
            "value": "value"
        }
    });

    let error = Action::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_set_variable_invalid_value() {
    let definition = json!({
        "type": "set_variable",
        "variable": "key",
        "value": "value"
    });

    let error = Action::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_set_variable_dependencies() {
    let definition = json!({
        "type": "set_variable",
        "variable": "key",
        "value": {
            "type": "output",
            "output_id": 20,
            "path": "/key"
        }
    });

    let action = Action::try_from(definition).unwrap();
    assert_eq!(action.dependencies(), vec![20]);
}
