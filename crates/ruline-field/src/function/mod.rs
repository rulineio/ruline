use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;
mod func;
mod validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Min,
    Max,
    Abs,
    Mean,
    Median,
    Upper,
    Lower,
    Join,
}

impl Function {
    pub fn process(&self, args: Vec<Value>) -> Result<Value> {
        match self {
            Function::Add => func::add(args),
            Function::Sub => func::sub(args),
            Function::Mul => func::mul(args),
            Function::Div => func::div(args),
            Function::Mod => func::mod_(args),
            Function::Pow => func::pow(args),
            Function::Min => func::min(args),
            Function::Max => func::max(args),
            Function::Abs => func::abs(args),
            Function::Mean => func::mean(args),
            Function::Median => func::median(args),
            Function::Upper => func::upper(args),
            Function::Lower => func::lower(args),
            Function::Join => func::join(args),
        }
    }
}
