use crate::input_utils::run_command::value::LispValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ListOp {
    List,
}

impl ListOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "list" => Some(Self::List),
            _ => None,
        }
    }
}

pub fn eval_list_op(op: &ListOp, args: &[String]) -> Result<LispValue, String> {
    match op {
        ListOp::List => {
            let items = args.iter().map(|s| LispValue::Str(s.clone())).collect();
            Ok(LispValue::List(items))
        }
    }
}
