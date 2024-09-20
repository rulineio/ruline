#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use insta::assert_snapshot;
use ruline_workflow::Workflow;
use serde::Deserialize;
use serde_json::json;

#[test]
fn test_workflow() {
    let data = json!({
        "name": "John",
        "age": 30,
    });

    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "name": "component_1",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "3"
                    ],
                    "results": [
                        "4"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "not_exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        },
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "not_exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "2": {
                "type": "condition",
                "name": "component_2",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "3"
                    ],
                    "results": [
                        "3"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "3": {
                "type": "condition",
                "name": "component_3",
                "definition": {
                    "type": "binary",
                    "fallbacks": [],
                    "results": [],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "4": {
                "type": "condition",
                "name": "component_4",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "6"
                    ],
                    "results": [
                        "5",
                        "6"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "not_equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 2
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "not_equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 2
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "5": {
                "type": "condition",
                "name": "gt_test",
                "definition": {
                    "type": "binary",
                    "id": "5",
                    "fallbacks": [],
                    "results": [
                        "6"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "greater_than",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        },
                        {
                            "id": "102",
                            "type": "comparison",
                            "operator": "greater_than",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "6": {
                "type": "condition",
                "name": "gte_test",
                "definition": {
                    "id": "100",
                    "type": "binary",
                    "fallbacks": [
                        "8"
                    ],
                    "results": [
                        "7"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "greater_than_or_equal",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "greater_than_or_equal",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "7": {
                "type": "action",
                "name": "set_color",
                "definition": {
                    "type": "set_variable",
                    "variable": "color",
                    "value": {
                        "type": "value",
                        "value": "green"
                    }
                }
            },
            "8": {
                "type": "action",
                "name": "set_color",
                "definition": {
                    "type": "set_variable",
                    "variable": "color",
                    "value": {
                        "type": "value",
                        "value": "red"
                    }
                }
            }
        }
    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    let output = json!({
        "color": {
            "type": "variable",
            "variable": "color"
        }
    });

    let workflow = Workflow::builder()
        .with_definition(definition)
        .with_output(output)
        .build()
        .unwrap();

    assert!(workflow.validate().is_ok());

    let result = workflow.process(data).unwrap();
    assert_eq!(
        result,
        json!({
            "color": "green"
        })
    );
}
#[test]
fn test_cyclic_workflow() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "name": "check_counter_less_than_3",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "2"
                    ],
                    "results": [
                        "3"
                    ],
                    "expression": {
                        "id": "100",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 3
                        }
                        ]
                    }
                }
            },
            "2": {
                "type": "condition",
                "name": "check_counter_less_than_5",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "1"
                    ],
                    "results": [
                        "3"
                    ],
                    "expression": {
                        "id": "101",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 5
                        }
                        ]
                    }
                }
            },
            "3": {
                "type": "condition",
                "name": "increment_counter",
                "definition": {
                    "type": "binary",
                    "fallbacks": [
                        "1"
                    ],
                    "results": [
                        "1"
                    ],
                    "expression": {
                        "id": "102",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "variable",
                                "variable": "counter"
                            }
                            ]
                        }
                        ]
                    }
                }
            }
        }
    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    let output = json!({
        "color": {
            "type": "variable",
            "variable": "color"
        }
    });

    let workflow = Workflow::builder()
        .with_definition(definition)
        .with_output(output)
        .build()
        .unwrap();

    assert_snapshot!(workflow.validate().unwrap_err().to_string());
}

#[test]
fn test_workflow_components_malformed() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "iamnotatype",
                "name": "check_counter_less_than_3",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "2" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "100",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 3
                        }
                        ]
                    }
                }
            }     
        }
    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .build()
        .unwrap_err()
        .to_string());
}

#[test]
fn test_workflow_output_malformed() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "name": "check_counter_less_than_3",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "2" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "100",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 3
                        }
                        ]
                    }
                }
            },
            "2": {
                "type": "condition",
                "name": "check_counter_less_than_5",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "1" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "101",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 5
                        }
                        ]
                    }
                }
            },
            "3": {
                "type": "condition",
                "name": "increment_counter",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "1" ],
                    "results": [ "1" ],
                    "expression": {
                        "id": "102",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "variable",
                                "variable": "counter"
                            }
                            ]
                        }
                        ]
                    }
                }
            }
        }
    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    let output = json!({
        "color": "white"
    });

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .with_output(output)
        .build()
        .unwrap_err()
        .to_string());
}

#[test]
fn test_workflow_component_name_missing() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "2" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "100",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 3
                        }
                        ]
                    }
                }
            },
            "2": {
                "type": "condition",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "1" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "101",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 5
                        }
                        ]
                    }
                }
            },
            "3": {
                "type": "condition",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "1" ],
                    "results": [ "1" ],
                    "expression": {
                        "id": "102",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "variable",
                                "variable": "counter"
                            }
                            ]
                        }
                        ]
                    }
                }
            }
        }
    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .build()
        .unwrap_err()
        .to_string());
}

#[test]
fn test_workflow_component_dependency_not_found() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "name": "component_1",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "3" ],
                    "results": [ "4" ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "not_exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        },
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "not_exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "2": {
                "type": "condition",
                "name": "component_2",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "3" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "exists",
                            "operands": [
                            {
                                "type": "value",
                                "value": null
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "3": {
                "type": "condition",
                "name": "component_3",
                "definition": {
                    "type": "binary",
                    "fallbacks": [],
                    "results": [],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "4": {
                "type": "condition",
                "name": "component_4",
                "definition": {
                    "type": "binary",
                    "fallbacks": [],
                    "results": [ "5" ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "not_equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 2
                            }
                            ]
                        },
                        {
                            "id": "100",
                            "type": "comparison",
                            "operator": "not_equals",
                            "operands": [
                            {
                                "type": "value",
                                "value": 1
                            },
                            {
                                "type": "value",
                                "value": 2
                            }
                            ]
                        }
                        ]
                    }
                }
            },
            "5": {
                "type": "condition",
                "name": "gt_test",
                "definition": {
                    "type": "binary",
                    "id": "5",
                    "fallbacks": [],
                    "results": [ "6" ],
                    "expression": {
                        "id": "100",
                        "type": "logical",
                        "operator": "and",
                        "expressions": [
                        {
                            "id": "101",
                            "type": "comparison",
                            "operator": "greater_than",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        },
                        {
                            "id": "102",
                            "type": "comparison",
                            "operator": "greater_than",
                            "operands": [
                            {
                                "type": "value",
                                "value": 2
                            },
                            {
                                "type": "value",
                                "value": 1
                            }
                            ]
                        }
                        ]
                    }
                }
            }
        }

    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    let output = json!({
        "color": {
            "type": "variable",
            "variable": "color"
        }
    });

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .with_output(output)
        .build()
        .unwrap_err()
        .to_string());
}

#[test]
fn test_workflow_component_missing_definition() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "type": "condition",
                "name": "component_1"
            }
        }

        "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .build()
        .unwrap_err()
        .to_string());
}

#[test]
fn test_workflow_component_missing_type() {
    let mut deserializer = serde_json::Deserializer::from_str(
        r#"
        {
            "1": {
                "name": "component_1",
                "definition": {
                    "type": "binary",
                    "fallbacks": [ "2" ],
                    "results": [ "3" ],
                    "expression": {
                        "id": "100",
                        "type": "comparison",
                        "operator": "less_than",
                        "operands": [
                        {
                            "type": "variable",
                            "variable": "counter"
                        },
                        {
                            "type": "value",
                            "value": 3
                        }
                        ]
                    }
                }
            }
        }

    "#,
    );
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let definition = serde_json::Value::deserialize(deserializer).unwrap();

    assert_snapshot!(Workflow::builder()
        .with_definition(definition)
        .build()
        .unwrap_err()
        .to_string());
}
