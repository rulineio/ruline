use anyhow::Result;
use serde_json::Value;

use super::error::ComparisonError;
use crate::{validate_min_operands, validate_operands};

pub fn equals(operands: &[Value]) -> Result<bool> {
    validate_min_operands!(operands, 2);

    let left = &operands[0];
    for right in operands.iter().skip(1) {
        if left != right {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn greater_than(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 2);

    let left = &operands[0];
    let right = &operands[1];
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            let right = right.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            Ok(left > right)
        }
        (Value::String(left), Value::String(right)) => Ok(left > right),
        (Value::Array(left), Value::Array(right)) => Ok(left.len() > right.len()),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn greater_than_or_equal(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 2);

    let left = &operands[0];
    let right = &operands[1];

    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            let right = right.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            Ok(left >= right)
        }
        (Value::String(left), Value::String(right)) => Ok(left >= right),
        (Value::Array(left), Value::Array(right)) => Ok(left.len() >= right.len()),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn less_than(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 2);

    let left = &operands[0];
    let right = &operands[1];

    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            let right = right.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            Ok(left < right)
        }
        (Value::String(left), Value::String(right)) => Ok(left < right),
        (Value::Array(left), Value::Array(right)) => Ok(left.len() < right.len()),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn less_than_or_equal(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 2);

    let left = &operands[0];
    let right = &operands[1];

    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            let right = right.as_f64().ok_or(ComparisonError::OperandTypeInvalid)?;
            Ok(left <= right)
        }
        (Value::String(left), Value::String(right)) => Ok(left <= right),
        (Value::Array(left), Value::Array(right)) => Ok(left.len() <= right.len()),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn empty(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 1);

    let operand = &operands[0];
    match operand {
        Value::Null => Ok(true),
        Value::Array(array) => Ok(array.is_empty()),
        Value::Object(object) => Ok(object.is_empty()),
        Value::String(string) => Ok(string.is_empty()),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn contains(operands: &[Value]) -> Result<bool> {
    validate_operands!(operands, 2);

    let value = &operands[0];
    let array = &operands[1];
    match array {
        Value::Array(array) => Ok(array.contains(value)),
        _ => Err(ComparisonError::OperandTypeInvalid.into()),
    }
}

pub fn exists(operands: &[Value]) -> Result<bool> {
    validate_min_operands!(operands, 1);

    Ok(!operands.iter().any(|operand| operand.is_null()))
}
