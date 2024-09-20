use anyhow::Result;
use ruline_context::Context;
use ruline_field::{Field, FieldDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;
pub use error::ActionError;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ActionDefinition {
    SetVariable {
        variable: String,
        value: FieldDefinition,
    },
}

#[derive(Debug)]
pub struct Action {
    pub definition: ActionDefinition,
    pub dependencies: Vec<String>,
}

impl TryFrom<ActionDefinition> for Action {
    type Error = ActionError;

    fn try_from(definition: ActionDefinition) -> Result<Self, Self::Error> {
        let dependencies = match &definition {
            ActionDefinition::SetVariable { value, .. } => Field::from(value).dependencies(),
        };

        Ok(Action {
            definition,
            dependencies,
        })
    }
}

impl TryFrom<Value> for Action {
    type Error = ActionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: ActionDefinition =
            serde_json::from_value(value).map_err(ActionError::Serde)?;
        Action::try_from(definition)
    }
}

impl Action {
    pub fn process(&self, ctx: &Context) -> Result<()> {
        match &self.definition {
            ActionDefinition::SetVariable { variable, value } => {
                let field = Field::from(value);
                ctx.set_variable(variable.to_owned(), field.process(ctx)?);
            }
        }

        Ok(())
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.to_owned()
    }
}
