use thiserror::Error;

use crate::FieldDefinition;

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("{}", match .0 {
        FieldDefinition::Variable { variable } => format!(
            "Variable `{}` not found",
            variable
        ),
        FieldDefinition::Data {  path } => format!(
            "`{}` in data not found",
            path
        ),
        FieldDefinition::Output {
            output_id,
            path,
        } => format!(
            "`{}` in output `{}` not found",
            path, output_id
        ),
        FieldDefinition::Value {  .. } => "Value not found".to_string(),
    })]
    FieldNotFound(FieldDefinition),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
