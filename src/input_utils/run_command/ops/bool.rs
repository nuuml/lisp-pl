use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Equal,
}

impl BoolOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ">" => Some(Self::GreaterThan),
            "<" => Some(Self::LessThan),
            ">=" => Some(Self::GreaterOrEqual),
            "<=" => Some(Self::LessOrEqual),
            "=" => Some(Self::Equal),
            _ => None,
        }
    }
}

pub fn eval_bool_op(op: &BoolOp, args: &[LispValue]) -> Result<LispValue, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };

    match op {
        BoolOp::GreaterThan => {
            require(2)?;
            let numbers = get_numbers(args)?;
            Ok(LispValue::Bool(numbers.windows(2).all(|pair| pair[0] > pair[1])))
        }
        BoolOp::LessThan => {
            require(2)?;
            let numbers = get_numbers(args)?;
            Ok(LispValue::Bool(numbers.windows(2).all(|pair| pair[0] < pair[1])))
        }
        BoolOp::GreaterOrEqual => {
            require(2)?;
            let numbers = get_numbers(args)?;
            Ok(LispValue::Bool(
                numbers.windows(2).all(|pair| pair[0] >= pair[1]),
            ))
        }
        BoolOp::LessOrEqual => {
            require(2)?;
            let numbers = get_numbers(args)?;
            Ok(LispValue::Bool(
                numbers.windows(2).all(|pair| pair[0] <= pair[1]),
            ))
        }
        BoolOp::Equal => {
            require(2)?;
            let first = &args[0];
            Ok(LispValue::Bool(
                args[1..].iter().all(|value| value == first),
            ))
        }
    }
}

fn get_numbers(args: &[LispValue]) -> Result<Vec<f64>, String> {
    args.iter()
        .map(|value| match value {
            LispValue::Number(number) => Ok(*number),
            _ => Err(format!("Expected number, got {value}")),
        })
        .collect::<Result<Vec<f64>, String>>()
}