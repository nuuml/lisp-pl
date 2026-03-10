#[derive(Debug, Clone, PartialEq)]
pub enum LispValue {
    Number(f64),
    Str(String),
    Bool(bool),
    List(Vec<LispValue>),
    Nil,
}

impl std::fmt::Display for LispValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LispValue::Number(n) => write!(f, "{n}"),
            LispValue::Str(s) => write!(f, "{s}"),
            LispValue::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            LispValue::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, ")")
            }
            LispValue::Nil => write!(f, ""),
        }
    }
}
