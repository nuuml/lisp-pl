pub fn run_command(input: Vec<String>) -> Result<f64, String> {
    let op = input
        .first()
        .and_then(|t| MathOp::from_str(t))
        .ok_or("Unknown or missing operator")?;

    let args: Vec<f64> = input[1..]
        .iter()
        .map(|t| t.parse::<f64>().map_err(|_| format!("Invalid number: {t}")))
        .collect::<Result<Vec<f64>, String>>()?;

    eval_op(&op, &args)
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
            "+"    => Some(Self::Add),
            "-"    => Some(Self::Subtract),
            "*"    => Some(Self::Multiply),
            "/"    => Some(Self::Divide),
            "%"    => Some(Self::Modulo),
            "**" | "^" => Some(Self::Power),
            "sqrt" => Some(Self::Sqrt),
            "abs"  => Some(Self::Abs),
            _      => None,
        }
    }
}

fn eval_op(op: &MathOp, args: &[f64]) -> Result<f64, String> {
    let require = |n: usize| -> Result<(), String> {
        if args.len() >= n {
            Ok(())
        } else {
            Err(format!("Expected {n} args, got {}", args.len()))
        }
    };

    match op {
        MathOp::Add      => Ok(args.iter().sum()),
        MathOp::Subtract => { require(2)?; Ok(args.iter().skip(1).fold(args[0], |acc, x| acc - x)) }
        MathOp::Multiply => Ok(args.iter().product()),
        MathOp::Divide   => { require(2)?; Ok(args.iter().skip(1).fold(args[0], |acc, x| acc / x)) }
        MathOp::Modulo   => { require(2)?; Ok(args[0] % args[1]) }
        MathOp::Power    => { require(2)?; Ok(args[0].powf(args[1])) }
        MathOp::Sqrt     => { require(1)?; Ok(args[0].sqrt()) }
        MathOp::Abs      => { require(1)?; Ok(args[0].abs()) }
    }
}