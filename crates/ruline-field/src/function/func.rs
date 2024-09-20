use anyhow::{Ok, Result};
use serde_json::Value;

use crate::{function::error::FunctionError, validate_args, validate_min_args};

macro_rules! vec_to_args {
    ($fn:ident, $args:expr) => {
        if $args.len() == 1 {
            match &$args[0] {
                Value::Array(arr) => return $fn(arr.to_vec()),
                _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
            }
        }
    };
}

pub fn add(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(add, args);

    let mut result = 0.0;
    for arg in args {
        result = match arg {
            Value::Number(n) => result + n.as_f64().unwrap(),
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }
    Ok(serde_json::to_value(result).map_err(FunctionError::Serde)?)
}

pub fn sub(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(sub, args);

    let mut result = 0.0;
    for (i, arg) in args.iter().enumerate() {
        result = match arg {
            Value::Number(n) if i == 0 => n.as_f64().unwrap(),
            Value::Number(n) => result - n.as_f64().unwrap(),
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }
    Ok(serde_json::to_value(result).map_err(FunctionError::Serde)?)
}

pub fn mul(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(mul, args);

    let mut result = 1.0;
    for arg in args {
        result = match arg {
            Value::Number(n) => result * n.as_f64().unwrap(),
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(result).map_err(FunctionError::Serde)?)
}

pub fn div(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(div, args);

    let mut result = 0.0;
    for (i, arg) in args.iter().enumerate() {
        result = match arg {
            Value::Number(n) if i == 0 => n.as_f64().unwrap(),
            Value::Number(n) => result / n.as_f64().unwrap(),
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(result).map_err(FunctionError::Serde)?)
}

pub fn mod_(args: Vec<Value>) -> Result<Value> {
    validate_args!(args, 2);

    let (a, b) = match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().unwrap(), b.as_f64().unwrap()),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    Ok(serde_json::to_value(a % b).map_err(FunctionError::Serde)?)
}

pub fn pow(args: Vec<Value>) -> Result<Value> {
    validate_args!(args, 2);

    let (a, b) = match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().unwrap(), b.as_f64().unwrap()),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    Ok(serde_json::to_value(a.powf(b)).map_err(FunctionError::Serde)?)
}

pub fn min(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(min, args);

    let mut min = f64::INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                min = min.min(n.as_f64().unwrap());
            }
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(min).map_err(FunctionError::Serde)?)
}

pub fn max(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(max, args);

    let mut max = f64::NEG_INFINITY;
    for arg in args {
        match arg {
            Value::Number(n) => {
                max = max.max(n.as_f64().unwrap());
            }
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(max).map_err(FunctionError::Serde)?)
}

pub fn abs(args: Vec<Value>) -> Result<Value> {
    validate_args!(args, 1);

    let arg = match &args[0] {
        Value::Number(n) => n.as_f64().unwrap(),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    Ok(serde_json::to_value(arg.abs()).map_err(FunctionError::Serde)?)
}

pub fn mean(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(mean, args);

    let mut sum = 0.0;
    for arg in args.iter() {
        match arg {
            Value::Number(n) => {
                sum += n.as_f64().unwrap();
            }
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(sum / args.len() as f64).map_err(FunctionError::Serde)?)
}

pub fn median(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 1);

    vec_to_args!(median, args);

    let mut numbers = vec![];
    for arg in args {
        match arg {
            Value::Number(n) => {
                numbers.push(n.as_f64().unwrap());
            }
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = numbers.len();
    let mid = len / 2;

    let mean = match len % 2 == 0 {
        true => (numbers[mid - 1] + numbers[mid]) / 2.0,
        false => numbers[mid],
    };

    Ok(serde_json::to_value(mean).map_err(FunctionError::Serde)?)
}

pub fn upper(args: Vec<Value>) -> Result<Value> {
    validate_args!(args, 1);

    let arg = match &args[0] {
        Value::String(s) => s.to_uppercase(),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    Ok(serde_json::to_value(arg).map_err(FunctionError::Serde)?)
}

pub fn lower(args: Vec<Value>) -> Result<Value> {
    validate_args!(args, 1);

    let arg = match &args[0] {
        Value::String(s) => s.to_lowercase(),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    Ok(serde_json::to_value(arg).map_err(FunctionError::Serde)?)
}

pub fn join(args: Vec<Value>) -> Result<Value> {
    validate_min_args!(args, 2);

    let separator = match &args[0] {
        Value::String(s) => s.to_owned(),
        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
    };

    let mut values = vec![];
    for arg in args.iter().skip(1) {
        match arg {
            Value::String(s) => values.push(s.to_owned()),
            Value::Array(arr) => {
                for value in arr {
                    match value {
                        Value::String(s) => values.push(s.to_owned()),
                        _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
                    }
                }
            }
            _ => return Err(FunctionError::ArgumentTypeInvalid.into()),
        }
    }

    Ok(serde_json::to_value(values.join(&separator)).map_err(FunctionError::Serde)?)
}
