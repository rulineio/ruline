use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::negate;

mod error;
mod negate;
mod predicate;
mod validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Contains,
    NotContains,
    Exists,
    NotExists,
    Empty,
    NotEmpty,
}

impl ComparisonOperator {
    pub fn eval(&self, operands: &[Value]) -> Result<bool> {
        match self {
            ComparisonOperator::Equals => predicate::equals(operands),
            ComparisonOperator::NotEquals => negate!(predicate::equals, operands),
            ComparisonOperator::GreaterThan => predicate::greater_than(operands),
            ComparisonOperator::GreaterThanOrEqual => predicate::greater_than_or_equal(operands),
            ComparisonOperator::LessThan => predicate::less_than(operands),
            ComparisonOperator::LessThanOrEqual => predicate::less_than_or_equal(operands),
            ComparisonOperator::Contains => predicate::contains(operands),
            ComparisonOperator::NotContains => negate!(predicate::contains, operands),
            ComparisonOperator::Exists => predicate::exists(operands),
            ComparisonOperator::NotExists => negate!(predicate::exists, operands),
            ComparisonOperator::Empty => predicate::empty(operands),
            ComparisonOperator::NotEmpty => negate!(predicate::empty, operands),
        }
    }
}
