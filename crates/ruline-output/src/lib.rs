use std::collections::HashMap;

use anyhow::Result;
use ruline_context::Context;
use ruline_field::{Field, FieldDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;
pub use error::OutputError;

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputDefinition(HashMap<String, FieldDefinition>);

#[derive(Debug)]
pub struct Output {
    pub definition: OutputDefinition,
}

impl Output {
    pub fn process(&self, ctx: &Context) -> Result<Value> {
        let mut output = HashMap::new();

        for (key, value) in &self.definition.0 {
            output.insert(key.clone(), Field::from(value).process(ctx)?);
        }
        Ok(serde_json::to_value(output)?)
    }
}

impl TryFrom<OutputDefinition> for Output {
    type Error = OutputError;

    fn try_from(definition: OutputDefinition) -> Result<Self, Self::Error> {
        Ok(Output { definition })
    }
}

impl TryFrom<Value> for Output {
    type Error = OutputError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: OutputDefinition =
            serde_json::from_value(value).map_err(OutputError::Serde)?;
        Output::try_from(definition)
    }
}
