pub mod bool;
pub mod list;
pub mod math;
pub mod string;

pub use bool::{BoolOp, eval_bool_op};
pub use list::{ListOp, eval_list_op};
pub use math::{MathOp, eval_math_op};
pub use string::{StringOp, eval_string_op};
