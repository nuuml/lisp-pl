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

pub fn eval_math_op(op: &MathOp, args: &[f64]) -> Result<LispValue, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };

    match op {
        MathOp::Add => Ok(LispValue::Number(args.iter().sum())),

        MathOp::Subtract => {
            require(2)?;
            Ok(LispValue::Number(
                args.iter().skip(1).fold(args[0], |acc, x| acc - x),
            ))
        }

        MathOp::Multiply => Ok(LispValue::Number(args.iter().product())),

        MathOp::Divide => {
            require(2)?;
            Ok(LispValue::Number(
                args.iter().skip(1).fold(args[0], |acc, x| acc / x),
            ))
        }

        MathOp::Modulo => {
            require(2)?;
            Ok(LispValue::Number(args[0] % args[1]))
        }

        MathOp::Power => {
            require(2)?;
            Ok(LispValue::Number(args[0].powf(args[1])))
        }

        MathOp::Sqrt => {
            require(1)?;
            Ok(LispValue::Number(args[0].sqrt()))
        }

        MathOp::Abs => {
            require(1)?;
            Ok(LispValue::Number(args[0].abs()))
        }
    }
}
