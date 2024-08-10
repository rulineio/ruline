use criterion::{criterion_group, criterion_main, Criterion};

use dashmap::DashMap;
use ruline_condition::Condition;
use ruline_context::Context;
use serde::Deserialize;
use serde_json::json;

pub fn benchmark(c: &mut Criterion) {
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
,
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
    let condition_definition = serde_json::Value::deserialize(deserializer).unwrap();

    let condition = Condition::try_from(condition_definition).unwrap();

    let data = json!({
        "first_value": 42,
        "second_value": 30,
    });
    let context = Context::new(data, DashMap::new());

    c.bench_function("process_condition", |b| {
        b.iter(|| condition.evaluate(&context))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
