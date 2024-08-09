#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_condition::Condition;
use ruline_context::Context;
use serde_json::json;

#[test]
fn test_greater_than() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 50
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 40
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 0.00401
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": 0.004
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "abcd"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "abcc"
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 11,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "id": 12,
                    "name": "value",
                    "value": [ "foo" ]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_less_than() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 40
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 0.00343
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": 0.00344
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "zzzx"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "zzzy"
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 11,
                    "name": "value",
                    "value": [ "foo" ]
                }, {
                    "type": "value",
                    "id": 12,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_equals() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "equals_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 30
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "foo"
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": { "foo": "bar" }
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": { "foo": "bar" }
                }]
            }, {
                "id": 306,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": true
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": true
                }]
            }, {
                "id": 307,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": null
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": null
                }]
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_not_equals() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "not_equals_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 40
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 20
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": 30
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "bar"
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": [ "foo", "baz" ]
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }]
            }, {
                "id": 306,
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": { "foo": "bar", "baz": "qux" }
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": { "foo": "bar" }
                }]
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_greater_than_or_equal() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 40
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 40
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 51
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": 50
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "foo"
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 11,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "id": 12,
                    "name": "value",
                    "value": [ "foo" ]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_less_than_or_equal() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 40
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 20
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": 21
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": "foo"
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 11,
                    "name": "value",
                    "value": [ "foo" ]
                }, {
                    "type": "value",
                    "id": 12,
                    "name": "value",
                    "value": [ "foo", "bar" ]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_contains() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "contains_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "contains",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": ["foo", "bar"]
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "contains",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 1
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": [1, 2, 3]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_not_contains() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "not_contains_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "not_contains",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": "baz"
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": ["foo", "bar"]
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "not_contains",
                "operands": [{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 0.4
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": [0.1, 0.2, 0.3]
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_not_exists() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "not_exists_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "not_exists",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": null
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "not_exists",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": null
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_exists() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "exists_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "exists",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 0
                }, {
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": []
                }, {
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": {}
                }, {
                    "type": "value",
                    "id": 10,
                    "name": "value",
                    "value": true
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "exists",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": ""
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "empty_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": []
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": ""
                }]
            }, {
                "id": 304,
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": {}
                }]
            }, {
                "id": 305,
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": null
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_not_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "not_empty_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": 302,
                "type": "comparison",
                "operator": "not_empty",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": ["foo"]
                }]
            }, {
                "id": 303,
                "type": "comparison",
                "operator": "not_empty",
                "operands": [{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": { "foo": "bar" }
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_equals_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "equals_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                }]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 35
                },{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 40
                }]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than",
                "operands": []
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_operand_types_mismatch() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": "30"
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_operand_type_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": true
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": true
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_or_equal_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 35
                },{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 40
                }]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_or_equal_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": []
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_or_equal_operand_types_mismatch() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": []
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_greater_than_or_equal_operand_type_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": true
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": true
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": 35
                },{
                    "type": "value",
                    "id": 9,
                    "name": "value",
                    "value": 40
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "less_than",
                "operands": []
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_operand_types_mismatch() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 30
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": "foo"
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_operand_type_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
                "id": 302,
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": {}
                },{
                    "type": "value",
                    "id": 8,
                    "name": "value",
                    "value": {}
                }
                ]
            }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_or_equal_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "id": 7,
                "name": "value",
                "value": 30
            },{
                "type": "value",
                "id": 8,
                "name": "value",
                "value": 35
            },{
                "type": "value",
                "id": 9,
                "name": "value",
                "value": 40
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_or_equal_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": []
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_or_equal_operand_types_mismatch() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "id": 7,
                "name": "value",
                "value": 30
            },{
                "type": "value",
                "id": 8,
                "name": "value",
                "value": "foo"
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_less_than_or_equal_operand_type_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "less_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "id": 7,
                "name": "value",
                "value": {}
            },{
                "type": "value",
                "id": 8,
                "name": "value",
                "value": {}
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_empty_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "empty_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "empty",
            "operands": [{
                "type": "value",
                "id": 7,
                "name": "value",
                "value": []
            },{
                "type": "value",
                "id": 8,
                "name": "value",
                "value": []
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_empty_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "empty_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "empty",
            "operands": []
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_contains_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "contains_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "contains",
            "operands": [{
                "type": "value",
                "id": 7,
                "name": "value",
                "value": "foo"
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_contains_operands_empty() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "contains_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "contains",
            "operands": []
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_contains_operand_type_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "contains_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "contains",
            "operands": [{
                "type": "value",
                "id": [7],
                "name": "value",
                "value": "foo"
            },{
                "type": "value",
                "id": 8,
                "name": "value",
                "value": "foo"
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_exists_operands_amount_invalid() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "exists_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 302,
            "type": "comparison",
            "operator": "exists",
            "operands": []
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let error = condition.evaluate(&context).unwrap_err();
    assert_snapshot!(error.to_string());
}
