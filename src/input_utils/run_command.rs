use std::str::FromStr;

pub fn run_command(input: Vec<String>) -> Result<Option<f64>, String> {
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
            eval_math_op(&op, &args).map(Some)
        }
        Command::String(op) => eval_string_op(&op, &input[1..]),
        Command::List(op) => eval_list_op(&op, &input[1..]),
        Command::Bool(op) => eval_bool_op(&op, &input),
    }
}

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
            return Some(Command::Bool(op))
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListOp {
    List
}

impl ListOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "list" => Some(ListOp::List),
            _ => None
        }
    }
}

fn eval_list_op(op: &ListOp, args: &[String]) -> Result<Option<f64>, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };
    match op {
        ListOp::List => {
            println!("List created");
            Ok(None)
        },
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum StringOp {
    Print,
    Println,
    StringEq,
    StringLen
}

impl StringOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "print" => Some(StringOp::Print),
            "println" => Some(StringOp::Println),
            "string=?" => Some(StringOp::StringEq),
            "string-length" => Some(StringOp::StringLen),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    True,
    False
}

impl BoolOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#t" => Some(BoolOp::True),
            "#f" => Some(BoolOp::False),
            _ => None
        }
    }
}

fn eval_bool_op(op: &BoolOp, args: &[String]) -> Result<Option<f64>, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", n))
        }
    };
    match op {
        BoolOp::True => Ok(Some(1.0)),
        BoolOp::False => Ok(Some(0.0)),
    }
}

fn eval_string_op(op: &StringOp, args: &[String]) -> Result<Option<f64>, String> {
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
            Ok(None)
        }
        StringOp::Println => {
            require(1)?;
            println!("{}", args[0]);
            Ok(None)
        }
        StringOp::StringEq => {
            require(2)?;
            if args[0] == args[1] {
                println!("#t");
                Ok(None)
            } else {
                println!("#f");
                Ok(None)
            }
        }
        StringOp::StringLen => {
            require(1)?;
            println!("{}", args[0].chars().count());
            Ok(None)
        }
    }
}

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

fn eval_math_op(op: &MathOp, args: &[f64]) -> Result<f64, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };

    match op {
        MathOp::Add => Ok(args.iter().sum()),

        MathOp::Subtract => {
            require(2)?;
            Ok(args.iter().skip(1).fold(args[0], |acc, x| acc - x))
        }

        MathOp::Multiply => Ok(args.iter().product()),

        MathOp::Divide => {
            require(2)?;
            Ok(args.iter().skip(1).fold(args[0], |acc, x| acc / x))
        }

        MathOp::Modulo => {
            require(2)?;
            Ok(args[0] % args[1])
        }

        MathOp::Power => {
            require(2)?;
            Ok(args[0].powf(args[1]))
        }

        MathOp::Sqrt => {
            require(1)?;
            Ok(args[0].sqrt())
        }

        MathOp::Abs => {
            require(1)?;
            Ok(args[0].abs())
        }
    }
}