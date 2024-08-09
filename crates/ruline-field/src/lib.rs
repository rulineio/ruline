use anyhow::Result;
use error::FieldError::{self, FieldNotFound};
use ruline_context::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum FieldDefinition {
    Variable { variable: String },
    Data { path: String },
    Output { output_id: i64, path: String },
    Value { value: Value },
}

impl FieldDefinition {
    pub fn get_type(&self) -> &'static str {
        match self {
            FieldDefinition::Variable { .. } => "variable",
            FieldDefinition::Data { .. } => "data",
            FieldDefinition::Output { .. } => "output",
            FieldDefinition::Value { .. } => "value",
        }
    }
}

pub struct Field {
    pub definition: FieldDefinition,
}

impl From<&FieldDefinition> for Field {
    fn from(definition: &FieldDefinition) -> Self {
        Field {
            definition: definition.clone(),
        }
    }
}

impl TryFrom<Value> for Field {
    type Error = FieldError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: FieldDefinition =
            serde_json::from_value(value).map_err(FieldError::Serde)?;

        Ok(Field::from(&definition))
    }
}

impl Field {
    pub fn process(&self, ctx: &Context) -> Result<Value> {
        let value = match &self.definition {
            FieldDefinition::Variable { variable, .. } => ctx.get_variable(variable),

            FieldDefinition::Data { path, .. } => ctx.get_data(path),

            FieldDefinition::Output {
                output_id, path, ..
            } => ctx.get_output(*output_id, path),

            FieldDefinition::Value { value, .. } => Some(value.clone()),
        };

        match value {
            Some(value) => Ok(value),
            None => Err(FieldNotFound(self.definition.clone()).into()),
        }
    }

    pub fn dependency(&self) -> i64 {
        match &self.definition {
            FieldDefinition::Output { output_id, .. } => *output_id,
            _ => 0,
        }
    }
}