use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    True,
    False,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Equal
}

impl BoolOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#t" => Some(Self::True),
            "#f" => Some(Self::False),
            ">"  => Some(Self::GreaterThan),
            "<"  => Some(Self::LessThan),
            ">=" => Some(Self::GreaterOrEqual),
            "<=" => Some(Self::LessOrEqual),
            "="  => Some(Self::Equal),
            _ => None,
        }
    }
}

pub fn eval_bool_op(op: &BoolOp, _args: &[String]) -> Result<LispValue, String> {
    let require = |n: usize| -> Result<(), String> {
        if _args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", _args.len()))
        }
    };
    match op {
        BoolOp::True => Ok(LispValue::Bool(true)),
        BoolOp::False => Ok(LispValue::Bool(false)),
        BoolOp::GreaterThan => {
            require(2)?;
            let a: f64 = _args[0].parse().unwrap();
            let b: f64 = _args[1].parse().unwrap();
            let result = a > b;
            Ok(LispValue::Bool(result))
        },
        BoolOp::LessThan => {
            require(2)?;
            let a: f64 = _args[0].parse().unwrap();
            let b: f64 = _args[1].parse().unwrap();
            let result = a < b;
            Ok(LispValue::Bool(result))
        },
        BoolOp::GreaterOrEqual => {
            require(2)?;
            let a: f64 = _args[0].parse().unwrap();
            let b: f64 = _args[1].parse().unwrap();
            let result = a >= b;
            Ok(LispValue::Bool(result))
        },
        BoolOp::LessOrEqual => {
            require(2)?;
            let a: f64 = _args[0].parse().unwrap();
            let b: f64 = _args[1].parse().unwrap();
            let result = a <= b;
            Ok(LispValue::Bool(result))
        },
        BoolOp::Equal => {
            require(2)?;
            let a: f64 = _args[0].parse().unwrap();
            let b: f64 = _args[1].parse().unwrap();
            let result = a == b;
            Ok(LispValue::Bool(result))
        }

    }
}
