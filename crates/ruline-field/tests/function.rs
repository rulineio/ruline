#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_context::Context;
use ruline_field::{assert_deserialize_error, assert_field, assert_field_error, Field};
use serde_json::json;

#[test]
fn test_function_field_nested() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "function",
            "function": "add",
            "args": [
            {
                "type": "value",
                "value": 1
            },
            {
                "type": "function",
                "function": "mul",
                "args": [
                {
                    "type": "value",
                    "value": 2
                },
                {
                    "type": "value",
                    "value": 3
                }
                ]
            }
            ]
        },
        {
            "type": "function",
            "function": "sub",
            "args": [
            {
                "type": "value",
                "value": 3
            },
            {
                "type": "value",
                "value": 4
            }
            ]
        }
        ]
    });

    assert_field!(&context, definition, 6.0);
}

#[test]
fn test_function_field_dependencies() {
    let definition = json!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "function",
            "function": "add",
            "args": [
            {
                "type": "function",
                "function": "add",
                "args": [
                {
                    "type": "output",
                    "output_id": 10,
                    "path": "/foo/bar/baz"
                },
                {
                    "type": "function",
                    "function": "mul",
                    "args": [
                    {
                        "type": "output",
                        "output_id": 20,
                        "path": "/foo/bar/baz"
                    },
                    {
                        "type": "value",
                        "value": 3
                    }
                    ]
                }
                ]
            },
            {
                "type": "function",
                "function": "sub",
                "args": [
                {
                    "type": "output",
                    "output_id": 40,
                    "path": "/foo/bar/baz"
                },
                {
                    "type": "value",
                    "value": 4
                }
                ]
            }
            ]
        }        ]
    });

    let field = Field::try_from(definition).unwrap();
    let dependencies = field.dependencies();
    assert_eq!(dependencies, vec![10, 20, 40]);
}

#[test]
fn test_function_invalid_function() {
    assert_deserialize_error!({
        "type": "function",
        "function": "invalid",
        "args": []
    });
}

#[test]
fn test_function_field_add() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "value",
            "value": 1
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });

    assert_field!(&context, definition, 3.0);
}

#[test]
fn test_function_field_add_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "value",
            "value": [1, 2, 3]
        }
        ]
    });

    assert_field!(&context, definition, 6.0);
}

#[test]
fn test_function_field_add_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_add_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "add",
        "args": []
    });
}

#[test]
fn test_function_field_add_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "add",
        "args": [
        {
            "type": "value",
            "value": ["foo", 2, 3]
        }
        ]
    });
}

#[test]
fn test_function_field_sub() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "sub",
        "args": [
        {
            "type": "value",
            "value": 4
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_sub_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "sub",
        "args": [
        {
            "type": "value",
            "value": [4, 2, 1]
        }
        ]
    });

    assert_field!(&context, definition, 1.0);
}

#[test]
fn test_function_field_sub_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "sub",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_sub_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "sub",
        "args": []
    });
}

#[test]
fn test_function_field_sub_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "sub",
        "args": [
        {
            "type": "value",
            "value": ["foo", 2, 3]
        }
        ]
    });
}

#[test]
fn test_function_field_mul() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "mul",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 6.0);
}

#[test]
fn test_function_field_mul_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "mul",
        "args": [
        {
            "type": "value",
            "value": [2, 3, 4]
        }
        ]
    });

    assert_field!(&context, definition, 24.0);
}

#[test]
fn test_function_field_mul_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "mul",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_mul_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "mul",
        "args": []
    });
}

#[test]
fn test_function_field_mul_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "mul",
        "args": [
        {
            "type": "value",
            "value": [true]
        }
        ]
    });
}

#[test]
fn test_function_field_div() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "div",
        "args": [
        {
            "type": "value",
            "value": 6
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_div_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "div",
        "args": [
        {
            "type": "value",
            "value": [6, 3, 2]
        }
        ]
    });

    assert_field!(&context, definition, 1.0);
}

#[test]
fn test_function_field_div_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "div",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_div_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "div",
        "args": []
    });
}

#[test]
fn test_function_field_div_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "div",
        "args": [
        {
            "type": "value",
            "value": [[1,2], [3,4]]
        }
        ]
    });
}

#[test]
fn test_function_field_mod() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "mod",
        "args": [
        {
            "type": "value",
            "value": 6
        },
        {
            "type": "value",
            "value": 4
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_mod_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "mod",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_mod_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "mod",
        "args": [
        {
            "type": "value",
            "value": 6
        }
        ]
    });
}

#[test]
fn test_function_field_pow() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "pow",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 8.0);
}

#[test]
fn test_function_field_pow_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "pow",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_pow_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "pow",
        "args": [
        {
            "type": "value",
            "value": 6
        },
        {
            "type": "value",
            "value": 4
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_min() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "min",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_min_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "min",
        "args": [
        {
            "type": "value",
            "value": [2, 3, 4]
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_min_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "min",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_min_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "min",
        "args": []
    });
}

#[test]
fn test_function_field_min_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "min",
        "args": [
        {
            "type": "value",
            "value": [{}, {}]
        }
        ]
    });
}

#[test]
fn test_function_field_max() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "max",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 3.0);
}

#[test]
fn test_function_field_max_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "max",
        "args": [
        {
            "type": "value",
            "value": [2, 3, 4]
        }
        ]
    });

    assert_field!(&context, definition, 4.0);
}

#[test]
fn test_function_field_max_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "max",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_max_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "max",
        "args": []
    });
}

#[test]
fn test_function_field_max_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "max",
        "args": [
        {
            "type": "value",
            "value": [{}, {}]
        }
        ]
    });
}

#[test]
fn test_function_field_abs() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "abs",
        "args": [
        {
            "type": "value",
            "value": -2
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_abs_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "abs",
        "args": [
        {
            "type": "value",
            "value": "foo"
        }
        ]
    });
}

#[test]
fn test_function_field_abs_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "abs",
        "args": []
    });
}

#[test]
fn test_function_field_mean() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "mean",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        }
        ]
    });

    assert_field!(&context, definition, 2.5);
}

#[test]
fn test_function_field_mean_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "mean",
        "args": [
        {
            "type": "value",
            "value": [2, 3, 4, 24]
        }
        ]
    });

    assert_field!(&context, definition, 8.25);
}

#[test]
fn test_function_field_mean_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "mean",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_mean_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "mean",
        "args": []
    });
}

#[test]
fn test_function_field_mean_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "mean",
        "args": [
        {
            "type": "value",
            "value": ["foo", "bar"]
        }
        ]
    });
}

#[test]
fn test_function_field_median() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "median",
        "args": [
        {
            "type": "value",
            "value": 2
        },
        {
            "type": "value",
            "value": 3
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });

    assert_field!(&context, definition, 2.0);
}

#[test]
fn test_function_field_median_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "median",
        "args": [
        {
            "type": "value",
            "value": [2, 3, 4, 5, 20, 32.0, 44.3, 55.0]
        }
        ]
    });

    assert_field!(&context, definition, 12.5);
}

#[test]
fn test_function_field_median_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "median",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": 2
        }
        ]
    });
}

#[test]
fn test_function_field_median_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "median",
        "args": []
    });
}

#[test]
fn test_function_field_median_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "median",
        "args": [
        {
            "type": "value",
            "value": ["foo", "bar"]
        }
        ]
    });
}

#[test]
fn test_function_field_upper() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "upper",
        "args": [
        {
            "type": "value",
            "value": "foo"
        }
        ]
    });

    assert_field!(&context, definition, "FOO");
}

#[test]
fn test_function_field_upper_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "upper",
        "args": [
        {
            "type": "value",
            "value": 1
        }
        ]
    });
}

#[test]
fn test_function_field_upper_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "upper",
        "args": [
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": "bar"
        }
        ]
    });
}

#[test]
fn test_function_field_lower() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "lower",
        "args": [
        {
            "type": "value",
            "value": "FOO"
        }
        ]
    });

    assert_field!(&context, definition, "foo");
}

#[test]
fn test_function_field_lower_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "lower",
        "args": [
        {
            "type": "value",
            "value": 1
        }
        ]
    });
}

#[test]
fn test_function_field_lower_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "lower",
        "args": []
    });
}

#[test]
fn test_function_field_join() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "join",
        "args": [
        {
            "type": "value",
            "value": ","
        },
        {
            "type": "value",
            "value": "foo"
        },
        {
            "type": "value",
            "value": "bar"
        }
        ]
    });

    assert_field!(&context, definition, "foo,bar");
}

#[test]
fn test_function_field_join_vec() {
    let context = Context::new(json!({}), DashMap::new());

    let definition = json!({
        "type": "function",
        "function": "join",
        "args": [
        {
            "type": "value",
            "value": "-"
        },
        {
            "type": "value",
            "value": ["foo", "bar", "baz"]
        }
        ]
    });

    assert_field!(&context, definition, "foo-bar-baz");
}

#[test]
fn test_function_field_join_invalid_arg_type() {
    assert_field_error!({
        "type": "function",
        "function": "join",
        "args": [
        {
            "type": "value",
            "value": 1
        },
        {
            "type": "value",
            "value": "foo"
        }
        ]
    });
}

#[test]
fn test_function_field_join_invalid_arg_count() {
    assert_field_error!({
        "type": "function",
        "function": "join",
        "args": [{
            "type": "value",
            "value": ","
        }]
    });
}

#[test]
fn test_function_field_join_invalid_arg_type_vec() {
    assert_field_error!({
        "type": "function",
        "function": "join",
        "args": [
        {
            "type": "value",
            "value": 24
        },
        {
            "type": "value",
            "value": [1, 2, 3]
        }
        ]
    });
}
