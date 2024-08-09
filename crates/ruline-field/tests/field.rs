#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_context::Context;
use ruline_field::Field;
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

    let field_definition = json!({
        "type": "data",
        "id": 1,
        "name": "baz",
        "path": "/foo/bar/baz"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context).unwrap();

    assert_eq!(result, json!(42));
}

#[test]
fn test_get_data_field_not_found() {
    let data = json!({});

    let context = Context::new(data, DashMap::new());

    let field_definition = json!({
        "type": "data",
        "id": 1,
        "name": "baz",
        "path": "/foo/bar/baz"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context);

    assert!(result.is_err());
    assert_snapshot!(result.unwrap_err().to_string());
}

#[test]
fn test_get_output_field() {
    let data = json!({});

    let context = Context::new(data, DashMap::new());
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

    let field_definition = json!({
        "type": "output",
        "id": 1,
        "name": "30_baz",
        "output_id": 30,
        "path": "/foo/bar/baz"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context).unwrap();

    assert_eq!(result, json!(["a", "b", "c"]));
}

#[test]
fn test_get_output_field_not_found() {
    let data = json!({});

    let context = Context::new(data, DashMap::new());

    let field_definition = json!({
        "type": "output",
        "id": 1,
        "name": "30_baz",
        "output_id": 30,
        "path": "/foo/bar/baz"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context);

    assert!(result.is_err());
    assert_snapshot!(result.unwrap_err().to_string());
}

#[test]
fn test_get_variable_field() {
    let data = json!({});
    let variables = DashMap::new();
    variables.insert("foo".to_string(), json!("bar"));

    let context = Context::new(data, variables);

    let field_definition = json!({
        "type": "variable",
        "id": 1,
        "name": "foo",
        "variable": "foo"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context).unwrap();

    assert_eq!(result, json!("bar"));
}

#[test]
fn test_get_variable_field_not_found() {
    let data = json!({});
    let variables = DashMap::new();

    let context = Context::new(data, variables);

    let field_definition = json!({
        "type": "variable",
        "id": 1,
        "name": "foo",
        "variable": "foo"
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context);

    assert!(result.is_err());
    assert_snapshot!(result.unwrap_err().to_string());
}

#[test]
fn test_get_value_field() {
    let data = json!({});

    let context = Context::new(data, DashMap::new());

    let field_definition = json!({
        "type": "value",
        "id": 1,
        "name": "baz",
        "value": null
    });

    let field = Field::try_from(field_definition).unwrap();
    let result = field.process(&context).unwrap();

    assert_eq!(result, json!(null));
}
