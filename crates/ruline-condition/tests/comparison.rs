#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_condition::{assert_comparison, assert_comparison_error, Condition};
use ruline_context::Context;
use serde_json::json;

#[test]
fn test_greater_than() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "value": 50
                }, {
                    "type": "value",
                    "value": 40
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "value": 0.00401
                }, {
                    "type": "value",
                    "value": 0.004
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "value": "abcd"
                }, {
                    "type": "value",
                    "value": "abcc"
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "value": [ "foo" ]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_less_than() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "value": 40
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "value": 0.00343
                }, {
                    "type": "value",
                    "value": 0.00344
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "value": "zzzx"
                }, {
                    "type": "value",
                    "value": "zzzy"
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "less_than",
                "operands": [{
                    "type": "value",
                    "value": [ "foo" ]
                }, {
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_equals() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "value": 30
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "value": "foo"
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": { "foo": "bar" }
                }, {
                    "type": "value",
                    "value": { "foo": "bar" }
                }]
            }, {
                "id": "306",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": true
                }, {
                    "type": "value",
                    "value": true
                }]
            }, {
                "id": "307",
                "type": "comparison",
                "operator": "equals",
                "operands": [{
                    "type": "value",
                    "value": null
                }, {
                    "type": "value",
                    "value": null
                }]
            }
            ]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_not_equals() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "value": 40
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "value": 20
                }, {
                    "type": "value",
                    "value": 30
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "value": "bar"
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "value": [ "foo", "baz" ]
                }, {
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }]
            }, {
                "id": "306",
                "type": "comparison",
                "operator": "not_equals",
                "operands": [{
                    "type": "value",
                    "value": { "foo": "bar", "baz": "qux" }
                }, {
                    "type": "value",
                    "value": { "foo": "bar" }
                }]
            }
            ]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_greater_than_or_equal() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": 40
                }, {
                    "type": "value",
                    "value": 40
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": 51
                }, {
                    "type": "value",
                    "value": 50
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "value": "foo"
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "greater_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }, {
                    "type": "value",
                    "value": [ "foo" ]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_less_than_or_equal() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": 30
                }, {
                    "type": "value",
                    "value": 40
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": 20
                }, {
                    "type": "value",
                    "value": 21
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "value": "foo"
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "less_than_or_equal",
                "operands": [{
                    "type": "value",
                    "value": [ "foo" ]
                }, {
                    "type": "value",
                    "value": [ "foo", "bar" ]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_contains() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "contains",
                "operands": [{
                    "type": "value",
                    "value": "foo"
                }, {
                    "type": "value",
                    "value": ["foo", "bar"]
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "contains",
                "operands": [{
                    "type": "value",
                    "value": 1
                }, {
                    "type": "value",
                    "value": [1, 2, 3]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_not_contains() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "not_contains",
                "operands": [{
                    "type": "value",
                    "value": "baz"
                }, {
                    "type": "value",
                    "value": ["foo", "bar"]
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "not_contains",
                "operands": [{
                    "type": "value",
                    "value": 0.4
                }, {
                    "type": "value",
                    "value": [0.1, 0.2, 0.3]
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_not_exists() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "not_exists",
                "operands": [{
                    "type": "value",
                    "value": null
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "not_exists",
                "operands": [{
                    "type": "value",
                    "value": null
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_exists() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "exists",
                "operands": [{
                    "type": "value",
                    "value": 0
                }, {
                    "type": "value",
                    "value": []
                }, {
                    "type": "value",
                    "value": {}
                }, {
                    "type": "value",
                    "value": true
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "exists",
                "operands": [{
                    "type": "value",
                    "value": ""
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "value": []
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "value": ""
                }]
            }, {
                "id": "304",
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "value": {}
                }]
            }, {
                "id": "305",
                "type": "comparison",
                "operator": "empty",
                "operands": [{
                    "type": "value",
                    "value": null
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_not_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "300",
            "type": "logical",
            "operator": "and",
            "expressions": [{
                "id": "302",
                "type": "comparison",
                "operator": "not_empty",
                "operands": [{
                    "type": "value",
                    "value": ["foo"]
                }]
            }, {
                "id": "303",
                "type": "comparison",
                "operator": "not_empty",
                "operands": [{
                    "type": "value",
                    "value": { "foo": "bar" }
                }]
            }]
        }
    });
    assert_comparison!(definition, vec!["1"]);
}

#[test]
fn test_equals_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "equals",
            "operands": [{
                "type": "value",
                "value": 30
            }]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": 35
            },{
                "type": "value",
                "value": 40
            }]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_operand_types_mismatch() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": "30"
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_operand_type_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than",
            "operands": [{
                "type": "value",
                "value": true
            },{
                "type": "value",
                "value": true
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_or_equal_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than_or_equal",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": 35
            },{
                "type": "value",
                "value": 40
            }]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_or_equal_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than_or_equal",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_or_equal_operand_types_mismatch() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than_or_equal",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": []
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_greater_than_or_equal_operand_type_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "greater_than_or_equal",
            "operands": [{
                "type": "value",
                "value": true
            },{
                "type": "value",
                "value": true
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": 35
            },{
                "type": "value",
                "value": 40
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_operand_types_mismatch() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": "foo"
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_operand_type_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than",
            "operands": [{
                "type": "value",
                "value": {}
            },{
                "type": "value",
                "value": {}
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_or_equal_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": 35
            },{
                "type": "value",
                "value": 40
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_or_equal_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_or_equal_operand_types_mismatch() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "value": 30
            },{
                "type": "value",
                "value": "foo"
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_less_than_or_equal_operand_type_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "less_than_or_equal",
            "operands": [{
                "type": "value",
                "value": {}
            },{
                "type": "value",
                "value": {}
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_empty_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "empty",
            "operands": [{
                "type": "value",
                "value": []
            },{
                "type": "value",
                "value": []
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_empty_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "empty",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_contains_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "contains",
            "operands": [{
                "type": "value",
                "value": "foo"
            }]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_contains_operands_empty() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "contains",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_contains_operand_type_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "contains",
            "operands": [{
                "type": "value",
                "value": "foo"
            },{
                "type": "value",
                "value": "foo"
            }
            ]
        }
    });
    assert_comparison_error!(definition);
}

#[test]
fn test_exists_operands_amount_invalid() {
    let definition = json!({
        "type": "binary",
        "fallbacks": [ "0" ],
        "results": [ "1" ],
        "expression": {
            "id": "302",
            "type": "comparison",
            "operator": "exists",
            "operands": []
        }
    });
    assert_comparison_error!(definition);
}
