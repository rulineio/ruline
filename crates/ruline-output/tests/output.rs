#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_context::Context;
use ruline_output::Output;
use serde_json::json;

#[test]
fn test_output() {
    let definition = json!({
        "key": {
            "type": "value",
            "value": "value"
        },
        "key2": {
            "type": "data",
            "path": "/my_data"
        },
        "key3": {
            "type": "variable",
            "variable": "var"
        }
    });

    let variables = DashMap::new();
    variables.insert("var".to_string(), json!("val"));
    let context = Context::new(
        json!({
            "my_data": "my_value"
        }),
        variables,
    );

    let output = Output::try_from(definition).unwrap();
    let result = output.process(&context).unwrap();

    assert_eq!(
        result,
        json!({
            "key": "value",
            "key2": "my_value",
            "key3": "val"
        })
    );
}

#[test]
fn test_output_invalid_field_type() {
    let definition = json!({
        "key": {
            "type": "invalid",
            "value": "value"
        }
    });

    let output = Output::try_from(definition);
    assert!(output.is_err());
    assert_snapshot!(output.unwrap_err().to_string());
}
