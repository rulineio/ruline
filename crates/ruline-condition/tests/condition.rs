#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_condition::{assert_condition_deserialize_error, Condition};
use ruline_context::Context;
use serde::Deserialize;
use serde_json::json;

#[test]
fn test_decision_complex_nested_all_passed() {
    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    let mut deserializer = serde_json::Deserializer::from_str(
        r#"{
            "type":"decision",
            "id":1,
            "name":"complex_nested",
            "fallbacks":[
                0
            ],
            "results":{
                "100":[
                    1
                ],
                "300":[
                    2
                ]
            },
            "expressions":[
            {
                "id":100,
                "type":"comparison",
                "operator":"equals",
                "operands":[
                {
                    "type":"data",
                    "path":"/first_value"
                },
                {
                    "type":"value",
                    "value": 42
                }
                ]
            },
            {
                "id":300,
                "type":"logical",
                "operator":"and",
                "expressions":[
                {
                    "id":301,
                    "type":"logical",
                    "operator":"and",
                    "expressions":[
                    {
                        "id":302,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/first_value"
                        },
                        {
                            "type":"value",
                            "value": 40
                        }
                        ]
                    },
                    {
                        "id":303,
                        "type":"comparison",
                        "operator":"less_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 40
                        }
                        ]
                    },
                    {
                        "id":304,
                        "type":"comparison",
                        "operator":"equals",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 30
                        }
                        ]
                    },
                    {
                        "id": 305,
                        "type": "logical",
                        "operator": "or",
                        "expressions": [
                        {
                            "id": 306,
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "data",
                                "id": 11,
                                "name": "first_value",
                                "path": "/first_value"
                            },
                            {
                                "type": "value",
                                "value": 42
                            }
                            ]
                        },
                        {
                            "id": 307,
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "data",
                                "id": 13,
                                "name": "second_value",
                                "path": "/second_value"
                            },
                            {
                                "type": "value",
                                "value": 30
                            }
                            ]
                        }
                        ]
                    }
                    ]
                },
                {
                    "id":304,
                    "type":"logical",
                    "operator":"or",
                    "expressions":[
                    {
                        "id":305,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/first_value"
                        },
                        {
                            "type":"value",
                            "value": 50
                        }
                        ]
                    },
                    {
                        "id":306,
                        "type":"logical",
                        "operator":"and",
                        "expressions":[
                        {
                            "id":307,
                            "type":"comparison",
                            "operator":"less_than",
                            "operands":[
                            {
                                "type":"data",
                                "path":"/second_value"
                            },
                            {
                                "type":"value",
                                "value": 50
                            }
                            ]
                        },
                        {
                            "id":308,
                            "type":"comparison",
                            "operator":"greater_than",
                            "operands":[
                            {
                                "type":"data",
                                "path":"/second_value"
                            },
                            {
                                "type":"value",
                                "value": 20
                            }
                            ]
                        }
                        ]
                    }
                    ]
                }
                ]
            }
            ]
        }"#,
    );

    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    let condition = Condition::try_from(definition).unwrap();
    assert!(condition.validate().is_ok());
    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_decision_complex_nested_some_pass() {
    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    let definition = json!({
        "type":"decision",
        "fallbacks":[
            0
        ],
        "results":{
            "100":[
                1
            ],
            "300":[
                2
            ]
        },
        "expressions":[
        {
            "id":100,
            "type":"comparison",
            "operator":"equals",
            "operands":[
            {
                "type":"data",
                "path":"/first_value"
            },
            {
                "type":"value",
                "value": 42
            }
            ]
        },
        {
            "id":300,
            "type":"logical",
            "operator":"and",
            "expressions":[
            {
                "id":301,
                "type":"logical",
                "operator":"and",
                "expressions":[
                {
                    "id":302,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                },
                {
                    "id":303,
                    "type":"comparison",
                    "operator":"less_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/second_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                }
                ]
            },
            {
                "id":304,
                "type":"logical",
                "operator":"or",
                "expressions":[
                {
                    "id":305,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 50
                    }
                    ]
                },
                {
                    "id":306,
                    "type":"logical",
                    "operator":"and",
                    "expressions":[
                    {
                        "id":307,
                        "type":"comparison",
                        "operator":"less_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 10
                        }
                        ]
                    },
                    {
                        "id":308,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 20
                        }
                        ]
                    }
                    ]
                }
                ]
            }
            ]
        }
        ]
    });

    let condition = Condition::try_from(definition).unwrap();
    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_decision_fallback() {
    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    let definition = json!({
        "type":"decision",
        "fallbacks":[
            0
        ],
        "results":{
            "100":[
                1
            ],
            "300":[
                2
            ]
        },
        "expressions":[
        {
            "id":100,
            "type":"comparison",
            "operator":"equals",
            "operands":[
            {
                "type":"data",
                "path":"/first_value"
            },
            {
                "type":"value",
                "value": 40
            }
            ]
        },
        {
            "id":300,
            "type":"logical",
            "operator":"and",
            "expressions":[
            {
                "id":301,
                "type":"logical",
                "operator":"and",
                "expressions":[
                {
                    "id":302,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                },
                {
                    "id":303,
                    "type":"comparison",
                    "operator":"less_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/second_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                }
                ]
            },
            {
                "id":304,
                "type":"logical",
                "operator":"or",
                "expressions":[
                {
                    "id":305,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 50
                    }
                    ]
                },
                {
                    "id":306,
                    "type":"logical",
                    "operator":"and",
                    "expressions":[
                    {
                        "id":307,
                        "type":"comparison",
                        "operator":"less_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 10
                        }
                        ]
                    },
                    {
                        "id":308,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 20
                        }
                        ]
                    }
                    ]
                }
                ]
            }
            ]
        }
        ]
    });

    let condition = Condition::try_from(definition).unwrap();
    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![0]);
}

#[test]
fn test_binary() {
    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    let definition = json!({
        "type":"binary",
        "fallbacks":[
            0
        ],
        "results": [ 1 ],
        "expression":{
            "id":300,
            "type":"logical",
            "operator":"and",
            "expressions":[
            {
                "id":301,
                "type":"logical",
                "operator":"and",
                "expressions":[
                {
                    "id":302,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                },
                {
                    "id":303,
                    "type":"comparison",
                    "operator":"less_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/second_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                }
                ]
            },
            {
                "id":304,
                "type":"logical",
                "operator":"or",
                "expressions":[
                {
                    "id":305,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 50
                    }
                    ]
                },
                {
                    "id":306,
                    "type":"logical",
                    "operator":"and",
                    "expressions":[
                    {
                        "id":307,
                        "type":"comparison",
                        "operator":"less_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 40
                        }
                        ]
                    },
                    {
                        "id":308,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 20
                        }
                        ]
                    }
                    ]
                }
                ]
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();
    assert!(condition.validate().is_ok());
    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_binary_fallback() {
    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    let definition = json!({
        "type":"binary",
        "fallbacks":[ 0 ],
        "results": [ 1, 2 ],
        "expression":{
            "id":300,
            "type":"logical",
            "operator":"and",
            "expressions":[
            {
                "id":301,
                "type":"logical",
                "operator":"and",
                "expressions":[
                {
                    "id":302,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                },
                {
                    "id":303,
                    "type":"comparison",
                    "operator":"less_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/second_value"
                    },
                    {
                        "type":"value",
                        "value": 40
                    }
                    ]
                }
                ]
            },
            {
                "id":304,
                "type":"logical",
                "operator":"or",
                "expressions":[
                {
                    "id":305,
                    "type":"comparison",
                    "operator":"greater_than",
                    "operands":[
                    {
                        "type":"data",
                        "path":"/first_value"
                    },
                    {
                        "type":"value",
                        "value": 50
                    }
                    ]
                },
                {
                    "id":306,
                    "type":"logical",
                    "operator":"and",
                    "expressions":[
                    {
                        "id":307,
                        "type":"comparison",
                        "operator":"less_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 10
                        }
                        ]
                    },
                    {
                        "id":308,
                        "type":"comparison",
                        "operator":"greater_than",
                        "operands":[
                        {
                            "type":"data",
                            "path":"/second_value"
                        },
                        {
                            "type":"value",
                            "value": 20
                        }
                        ]
                    }
                    ]
                }
                ]
            }
            ]
        }
    });

    let condition = Condition::try_from(definition).unwrap();
    let result = condition.evaluate(&context).unwrap();
    assert_eq!(result, vec![0]);
}

#[test]
fn test_dependencies() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": [
            {
                "id": 100,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [
                    {
                        "type": "data",
                        "path": "/first_value"
                    },
                    {
                        "type": "output",
                        "path": "/output",
                        "output_id": 14,
                    }
                ]
            }
        ]
    });
    let condition = Condition::try_from(definition).unwrap();
    let dependencies = condition.dependencies();
    assert_eq!(dependencies, vec![14]);
}

#[test]
fn test_dependants() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": [
            {
                "id": 100,
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [
                    {
                        "type": "data",
                        "path": "/first_value"
                    },
                    {
                        "type": "output",
                        "path": "/output",
                        "output_id": 14,
                    }
                ]
            }
        ]
    });
    let condition = Condition::try_from(definition).unwrap();
    let dependants = condition.dependants();
    assert_eq!(dependants, vec![0, 1]);
}

#[test]
fn test_expression_invalid_decision_expressions_empty() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": []
    });
    let condition = Condition::try_from(definition).unwrap();
    assert!(condition.validate().is_err());
    assert_snapshot!(condition.validate().unwrap_err().to_string())
}

#[test]
fn test_logical_child_missing() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": []
        }
    });
    let condition = Condition::try_from(definition).unwrap();
    assert!(condition.validate().is_err());
    assert_snapshot!(condition.validate().unwrap_err().to_string())
}

#[test]
fn test_logical_count_invalid() {
    let definition = json!({
        "type": "binary",
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
                    "value": 40
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();
    assert!(condition.validate().is_err());
    assert_snapshot!(condition.validate().unwrap_err().to_string())
}

#[test]
fn test_expression_invalid_decision_expressions_mismatch() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": {}
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_binary_error_expression_type() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": []
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_binary_error_expression_missing() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1]
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_decision_error_expressions_mismatch() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": [1],
        "expressions": {}
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_decision_error_expressions_missing() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": [1]
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_malformed() {
    let definition = json!({
        "type": "decision",
        "fallbacks": [0],
        "results": [1],
        "expressions": [],
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_type() {
    let definition = json!({
        "type": "invalid",
        "fallbacks": [0],
        "results": [1],
        "expressions": [],
        "expression": {}
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_fallbacks() {
    let definition = json!({
        "type": "binary",
        "fallbacks": {},
        "results": [1],
        "expression": {}
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_results() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": {},
        "expression": {}
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_comparison_operator() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": "invalid",
            "operands": []
        }
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_comparison_operator_type() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": 1,
            "operands": []
        }
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_comparison_operands_type() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": "greater_than",
            "operands": {}
        }
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_logical_operator() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "invalid",
            "expressions": []
        }
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_logical_operator_type() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": 1,
            "expressions": []
        }
    });
    assert_condition_deserialize_error!(definition);
}

#[test]
fn test_deserialize_error_invalid_logical_expressions_type() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "and",
            "expressions": {}
        }
    });
    assert_condition_deserialize_error!(definition);
}
