use anyhow::Result;
use error::FieldError::{self, FieldNotFound};
use function::Function;
use ruline_context::Context;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

mod error;
mod function;
mod test;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum FieldDefinition {
    Variable { variable: String },
    Data { path: String },
    Output { output_id: String, path: String },
    Value { value: Value },
    Function { function: Function, args: Vec<Self> },
}

#[derive(Debug)]
pub struct Field {
    pub definition: FieldDefinition,
}

impl From<&FieldDefinition> for Field {
    fn from(definition: &FieldDefinition) -> Self {
        Field {
            definition: definition.to_owned(),
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
            } => ctx.get_output(output_id, path),

            FieldDefinition::Value { value, .. } => match value {
                Value::Array(values) => {
                    let values = values
                        .iter()
                        .map(|value| {
                            Self::try_from(value.to_owned())
                                .map(|field| field.process(ctx))
                                .unwrap_or(Ok(value.to_owned()))
                        })
                        .collect::<Result<Vec<Value>>>()?;
                    return Ok(Value::Array(values));
                }
                Value::Object(map) => {
                    let map = map
                        .iter()
                        .map(|(key, value)| {
                            Self::try_from(value.to_owned())
                                .map(|field| field.process(ctx))
                                .unwrap_or(Ok(value.to_owned()))
                                .map(|value| (key.to_owned(), value))
                        })
                        .collect::<Result<Map<String, Value>>>()?;
                    return Ok(Value::Object(map));
                }
                _ => Some(value.to_owned()),
            },
            FieldDefinition::Function { function, args } => {
                let args = args
                    .iter()
                    .map(|arg| Self::from(arg).process(ctx))
                    .collect::<Result<Vec<Value>>>()?;
                function.process(args).map(Some)?
            }
        };

        match value {
            Some(value) => Ok(value),
            None => Err(FieldNotFound(self.definition.to_owned()).into()),
        }
    }

    pub fn dependencies(&self) -> Vec<String> {
        match &self.definition {
            FieldDefinition::Function { args, .. } => args
                .iter()
                .flat_map(|arg| Self::from(arg).dependencies())
                .collect(),
            FieldDefinition::Output { output_id, .. } => vec![output_id.to_owned()],
            _ => vec![],
        }
    }
}
