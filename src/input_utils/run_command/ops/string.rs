use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum StringOp {
    Print,
    Println,
    StringEq,
    StringLen,
}

impl StringOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "string=?" => Some(Self::StringEq),
            "string-length" => Some(Self::StringLen),
            _ => None,
        }
    }
}

pub fn eval_string_op(op: &StringOp, args: &[String]) -> Result<LispValue, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };

    match op {
        StringOp::Print => {
            require(1)?;
            print!("{}", args[0]);
            Ok(LispValue::Nil)
        }
        StringOp::Println => {
            require(1)?;
            println!("{}", args[0]);
            Ok(LispValue::Nil)
        }
        StringOp::StringEq => {
            require(2)?;
            Ok(LispValue::Bool(args[0] == args[1]))
        }
        StringOp::StringLen => {
            require(1)?;
            Ok(LispValue::Number(args[0].chars().count() as f64))
        }
    }
}
