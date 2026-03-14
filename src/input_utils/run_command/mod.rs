pub mod ops;
mod parser;
pub mod value;

use ops::{
    eval_bool_op, eval_list_op, eval_math_op, eval_string_op, BoolOp, ListOp, MathOp, StringOp,
};
use parser::{parse_expression, Expr};
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

pub fn run_command(input: &str) -> Result<LispValue, String> {
    let expression = parse_expression(input)?;
    eval_expression(&expression)
}

fn eval_expression(expr: &Expr) -> Result<LispValue, String> {
    match expr {
        Expr::Number(n) => Ok(LispValue::Number(*n)),
        Expr::Str(s) => Ok(LispValue::Str(s.clone())),
        Expr::Bool(b) => Ok(LispValue::Bool(*b)),
        Expr::Symbol(symbol) => Err(format!("Unknown symbol: {symbol}")),
        Expr::List(items) => eval_list_expression(items),
    }
}

fn eval_list_expression(items: &[Expr]) -> Result<LispValue, String> {
    if items.is_empty() {
        return Err("Cannot evaluate an empty list".to_string());
    }

    let command_name = match &items[0] {
        Expr::Symbol(symbol) => symbol,
        _ => return Err("First element in a list must be a command symbol".to_string()),
    };

    let command =
        Command::from_str(command_name).ok_or_else(|| format!("Unknown command: {command_name}"))?;

    let args = items[1..]
        .iter()
        .map(eval_expression)
        .collect::<Result<Vec<LispValue>, String>>()?;

    match command {
        Command::Math(op) => eval_math_op(&op, &args),
        Command::String(op) => eval_string_op(&op, &args),
        Command::List(op) => eval_list_op(&op, &args),
        Command::Bool(op) => eval_bool_op(&op, &args),
    }
}
