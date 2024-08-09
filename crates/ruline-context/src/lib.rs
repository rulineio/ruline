use dashmap::DashMap;
use serde_json::Value;

pub struct Context {
    pub data: Value,
    pub outputs: DashMap<i64, Value>,
    pub variables: DashMap<String, Value>,
}

impl Context {
    pub fn new(data: Value, variables: DashMap<String, Value>) -> Self {
        Self {
            data,
            outputs: DashMap::new(),
            variables,
        }
    }

    pub fn set_output(&self, id: i64, value: Value) {
        self.outputs.insert(id, value);
    }

    pub fn get_data(&self, key: &str) -> Option<Value> {
        self.data.pointer(key).cloned()
    }

    pub fn get_output(&self, id: i64, key: &str) -> Option<Value> {
        self.outputs.get(&id).and_then(|v| v.pointer(key).cloned())
    }

    pub fn get_variable(&self, key: &str) -> Option<Value> {
        self.variables.get(key).map(|v| v.value().clone())
    }
}
