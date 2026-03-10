pub mod ops;
pub mod value;

use ops::{
    BoolOp, ListOp, MathOp, StringOp,
    eval_bool_op, eval_list_op, eval_math_op, eval_string_op,
};
pub use value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Math(MathOp),
    String(StringOp),
    List(ListOp),
    Bool(BoolOp),
    // System(SystemOp),
}

impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Some(op) = MathOp::from_str(s) {
            return Some(Command::Math(op));
        }
        if let Some(op) = StringOp::from_str(s) {
            return Some(Command::String(op));
        }
        if let Some(op) = ListOp::from_str(s) {
            return Some(Command::List(op));
        }
        if let Some(op) = BoolOp::from_str(s) {
            return Some(Command::Bool(op));
        }
        None
    }
}

pub fn run_command(input: Vec<String>) -> Result<LispValue, String> {
    let cmd = input
        .first()
        .and_then(|t| Command::from_str(t))
        .ok_or("Unknown or missing command")?;

    match cmd {
        Command::Math(op) => {
            let args = input[1..]
                .iter()
                .map(|t| t.parse::<f64>().map_err(|_| format!("Invalid number: {t}")))
                .collect::<Result<Vec<f64>, String>>()?;
            eval_math_op(&op, &args)
        }
        Command::String(op) => eval_string_op(&op, &input[1..]),
        Command::List(op) => eval_list_op(&op, &input[1..]),
        Command::Bool(op) => eval_bool_op(&op, &input),
    }
}
