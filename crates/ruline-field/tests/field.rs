#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_context::Context;
use ruline_field::{assert_deserialize_error, assert_field, assert_field_error, Field};
use serde_json::json;

#[test]
fn test_get_data_field() {
    let data = json!({
        "foo": {
            "bar": {
                "baz": 42
            }
        }
    });
    let context = Context::new(data, DashMap::new());

    let definition = json!({
        "type": "data",
        "path": "/foo/bar/baz"
    });

    assert_field!(&context, definition, 42);
}

#[test]
fn test_get_data_field_not_found() {
    assert_field_error!({
        "type": "data",
        "path": "/foo/bar/baz"
    });
}

#[test]
fn test_get_output_field() {
    let context = Context::new(json!({}), DashMap::new());
    context.set_output(
        30,
        json!({
            "foo": {
                "bar": {
                    "baz": [ "a", "b", "c" ]
                }
            }
        }),
    );

    let definition = json!({
        "type": "output",
        "output_id": 30,
        "path": "/foo/bar/baz"
    });

    assert_field!(&context, definition, ["a", "b", "c"]);
}

#[test]
fn test_get_output_field_not_found() {
    assert_field_error!({
        "type": "output",
        "output_id": 30,
        "path": "/foo/bar/baz"
    });
}

#[test]
fn test_get_variable_field() {
    let variables = DashMap::new();
    variables.insert("foo".to_string(), json!("bar"));
    let context = Context::new(json!({}), variables);

    let definition = json!({
        "type": "variable",
        "variable": "foo"
    });

    assert_field!(&context, definition, "bar");
}

#[test]
fn test_get_variable_field_not_found() {
    assert_field_error!({
        "type": "variable",
        "variable": "foo"
    });
}

#[test]
fn test_get_value_field() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "value",
        "value": 42
    });

    assert_field!(&context, definition, null);
}

#[test]
fn test_serialization_invalid_field_type() {
    assert_deserialize_error!({ "type": "invalid" });
}

#[test]
fn test_serialization_invalid_type() {
    assert_deserialize_error!({ "type": "data", "path": 42 });
}

#[test]
fn get_dependency() {
    let definition = json!({
        "type": "output",
        "output_id": 30,
        "path": "/foo/bar/baz"
    });

    let field = Field::try_from(definition).unwrap();
    let result = field.dependency();
    assert_eq!(result, 30);
}
