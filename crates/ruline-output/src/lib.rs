use std::collections::HashMap;

use anyhow::Result;
use error::OutputError;
use ruline_context::Context;
use ruline_field::{Field, FieldDefinition};
use serde_json::Value;

mod error;

#[derive(Debug)]
pub struct Output {
    pub definition: HashMap<String, FieldDefinition>,
}

impl TryFrom<Value> for Output {
    type Error = OutputError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: HashMap<String, FieldDefinition> =
            serde_json::from_value(value).map_err(OutputError::Serde)?;

        Ok(Output { definition })
    }
}

impl Output {
    pub fn process(&self, ctx: &Context) -> Result<Value> {
        let mut output = HashMap::new();

        for (key, value) in &self.definition {
            output.insert(key.clone(), Field::from(value).process(ctx)?);
        }

        Ok(serde_json::to_value(output)?)
    }
}
