use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    True,
    False,
}

impl BoolOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#t" => Some(Self::True),
            "#f" => Some(Self::False),
            _ => None,
        }
    }
}

pub fn eval_bool_op(op: &BoolOp, _args: &[String]) -> Result<LispValue, String> {
    match op {
        BoolOp::True => Ok(LispValue::Bool(true)),
        BoolOp::False => Ok(LispValue::Bool(false)),
    }
}
