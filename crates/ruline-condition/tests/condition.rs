#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use dashmap::DashMap;
use insta::assert_snapshot;
use ruline_condition::Condition;
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
                   "id":1,
                   "name":"first_value",
                   "path":"/first_value"
                },
                {
                   "type":"value",
                   "id":2,
                   "name":"value",
                   "value":42
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
                               "id":7,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":8,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":30
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
                                          "id": 12,
                                          "name": "value",
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
                                          "id": 14,
                                          "name": "value",
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
                               "id":11,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":12,
                               "name":"value",
                               "value":50
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
                                     "id":13,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":14,
                                     "name":"value",
                                     "value":50
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
                                     "id":15,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":16,
                                     "name":"value",
                                     "value":20
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
                   "id":1,
                   "name":"first_value",
                   "path":"/first_value"
                },
                {
                   "type":"value",
                   "id":2,
                   "name":"value",
                   "value":42
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
                               "id":7,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":8,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":40
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
                               "id":11,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":12,
                               "name":"value",
                               "value":50
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
                                     "id":13,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":14,
                                     "name":"value",
                                     "value":10
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
                                     "id":15,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":16,
                                     "name":"value",
                                     "value":20
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
                   "id":1,
                   "name":"first_value",
                   "path":"/first_value"
                },
                {
                   "type":"value",
                   "id":2,
                   "name":"value",
                   "value":40
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
                               "id":7,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":8,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":40
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
                               "id":11,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":12,
                               "name":"value",
                               "value":50
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
                                     "id":13,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":14,
                                     "name":"value",
                                     "value":10
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
                                     "id":15,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":16,
                                     "name":"value",
                                     "value":20
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
       "id":1,
       "name":"complex_nested",
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
                               "id":7,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":8,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":40
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
                               "id":11,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":12,
                               "name":"value",
                               "value":50
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
                                     "id":13,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":14,
                                     "name":"value",
                                     "value":40
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
                                     "id":15,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":16,
                                     "name":"value",
                                     "value":20
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
       "id":1,
       "name":"complex_nested",
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
                               "id":7,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":8,
                               "name":"value",
                               "value":40
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
                               "id":9,
                               "name":"second_value",
                               "path":"/second_value"
                            },
                            {
                               "type":"value",
                               "id":10,
                               "name":"value",
                               "value":40
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
                               "id":11,
                               "name":"first_value",
                               "path":"/first_value"
                            },
                            {
                               "type":"value",
                               "id":12,
                               "name":"value",
                               "value":50
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
                                     "id":13,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":14,
                                     "name":"value",
                                     "value":10
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
                                     "id":15,
                                     "name":"second_value",
                                     "path":"/second_value"
                                  },
                                  {
                                     "type":"value",
                                     "id":16,
                                     "name":"value",
                                     "value":20
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
fn test_expression_invalid_decision_expressions_empty() {
    let definition = json!({
        "type": "decision",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": []
    });

    let condition = Condition::try_from(definition).unwrap();

    let err = condition.validate().unwrap_err();
    assert_snapshot!(err.to_string())
}

#[test]
fn test_expression_invalid_decision_expressions_mismatch() {
    let definition = json!({
        "type": "decision",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": {
            "100": [1]
        },
        "expressions": {}
    });

    let err = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(err.to_string())
}

#[test]
fn test_logical_child_missing() {
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
            "expressions": []
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let err = condition.validate().unwrap_err();
    assert_snapshot!(err.to_string())
}

#[test]
fn test_logical_count_invalid() {
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
                "operator": "greater_than",
                "operands": [{
                    "type": "value",
                    "id": 7,
                    "name": "value",
                    "value": 40
                }]
            }]
        }
    });

    let condition = Condition::try_from(definition).unwrap();

    let err = condition.validate().unwrap_err();
    assert_snapshot!(err.to_string())
}

#[test]
fn test_deserialize_binary_error_expression_type() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": []
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_binary_error_expression_missing() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1]
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_decision_error_expressions_mismatch() {
    let definition = json!({
        "type": "decision",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expressions": {}
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_decision_error_expressions_missing() {
    let definition = json!({
        "type": "decision",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1]
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_malformed() {
    let definition = json!({
        "type": "decision",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expressions": [],
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_type() {
    let definition = json!({
        "type": "invalid",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expressions": [],
        "expression": {}
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_fallbacks() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": {},
        "results": [1],
        "expression": {}
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_results() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": {},
        "expression": {}
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_comparison_operator() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": "invalid",
            "operands": []
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_comparison_operator_type() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": 1,
            "operands": []
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_comparison_operands_type() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "comparison",
            "operator": "greater_than",
            "operands": {}
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_logical_operator() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": "invalid",
            "expressions": []
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_logical_operator_type() {
    let definition = json!({
        "type": "binary",
        "id": 1,
        "name": "greater_than_or_equal_test",
        "fallbacks": [0],
        "results": [1],
        "expression": {
            "id": 300,
            "type": "logical",
            "operator": 1,
            "expressions": []
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}

#[test]
fn test_deserialize_error_invalid_logical_expressions_type() {
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
            "expressions": {}
        }
    });

    let error = Condition::try_from(definition).unwrap_err();
    assert_snapshot!(error.to_string());
}
