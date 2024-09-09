use anyhow::Result;
use error::ActionError;
use ruline_context::Context;
use ruline_field::{Field, FieldDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;

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
}

impl From<&ActionDefinition> for Action {
    fn from(definition: &ActionDefinition) -> Self {
        Action {
            definition: definition.clone(),
        }
    }
}

impl TryFrom<Value> for Action {
    type Error = ActionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let definition: ActionDefinition =
            serde_json::from_value(value).map_err(ActionError::Serde)?;

        Ok(Action::from(&definition))
    }
}

impl Action {
    pub fn process(&self, ctx: &Context) -> Result<()> {
        match &self.definition {
            ActionDefinition::SetVariable { variable, value } => {
                let field = Field::from(value);
                ctx.set_variable(variable.clone(), field.process(ctx)?);
            }
        }

        Ok(())
    }
}
