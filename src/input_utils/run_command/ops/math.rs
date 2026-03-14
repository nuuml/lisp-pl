use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Sqrt,
    Abs,
}

impl MathOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            "%" => Some(Self::Modulo),
            "**" | "^" => Some(Self::Power),
            "sqrt" => Some(Self::Sqrt),
            "abs" => Some(Self::Abs),
            _ => None,
        }
    }
}

pub fn eval_math_op(op: &MathOp, args: &[LispValue]) -> Result<LispValue, String> {
    let numbers = args
        .iter()
        .map(expect_number)
        .collect::<Result<Vec<f64>, String>>()?;

    let require = |n: usize| -> Result<(), String> {
        if numbers.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", numbers.len()))
        }
    };

    match op {
        MathOp::Add => Ok(LispValue::Number(numbers.iter().sum())),

        MathOp::Subtract => {
            require(2)?;
            Ok(LispValue::Number(
                numbers.iter().skip(1).fold(numbers[0], |acc, x| acc - x),
            ))
        }

        MathOp::Multiply => Ok(LispValue::Number(numbers.iter().product())),

        MathOp::Divide => {
            require(2)?;
            Ok(LispValue::Number(
                numbers.iter().skip(1).fold(numbers[0], |acc, x| acc / x),
            ))
        }

        MathOp::Modulo => {
            require(2)?;
            Ok(LispValue::Number(numbers[0] % numbers[1]))
        }

        MathOp::Power => {
            require(2)?;
            Ok(LispValue::Number(numbers[0].powf(numbers[1])))
        }

        MathOp::Sqrt => {
            require(1)?;
            Ok(LispValue::Number(numbers[0].sqrt()))
        }

        MathOp::Abs => {
            require(1)?;
            Ok(LispValue::Number(numbers[0].abs()))
        }
    }
}

fn expect_number(value: &LispValue) -> Result<f64, String> {
    match value {
        LispValue::Number(number) => Ok(*number),
        _ => Err(format!("Expected number, got {value}")),
    }
}
