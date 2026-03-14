#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    List(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Atom { text: String, quoted: bool },
}

pub fn parse_expression(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err("Empty input".to_string());
    }

    let mut cursor = 0;
    let expression = parse_expr(&tokens, &mut cursor)?;

    if cursor != tokens.len() {
        return Err("Unexpected tokens after expression".to_string());
    }

    Ok(expression)
}

fn parse_expr(tokens: &[Token], cursor: &mut usize) -> Result<Expr, String> {
    match tokens.get(*cursor) {
        Some(Token::LParen) => {
            *cursor += 1;
            let mut items = Vec::new();

            while !matches!(tokens.get(*cursor), Some(Token::RParen)) {
                if *cursor >= tokens.len() {
                    return Err("Missing closing ')'".to_string());
                }
                items.push(parse_expr(tokens, cursor)?);
            }

            *cursor += 1;
            Ok(Expr::List(items))
        }
        Some(Token::RParen) => Err("Unexpected ')'".to_string()),
        Some(Token::Atom { text, quoted }) => {
            *cursor += 1;
            if *quoted {
                Ok(Expr::Str(text.clone()))
            } else {
                Ok(parse_unquoted_atom(text))
            }
        }
        None => Err("Unexpected end of input".to_string()),
    }
}

fn parse_unquoted_atom(text: &str) -> Expr {
    if text == "#t" {
        Expr::Bool(true)
    } else if text == "#f" {
        Expr::Bool(false)
    } else if let Ok(number) = text.parse::<f64>() {
        Expr::Number(number)
    } else {
        Expr::Symbol(text.to_string())
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.next() {
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' | '\'' => {
                let quote = ch;
                let mut text = String::new();
                let mut escaped = false;
                let mut closed = false;

                while let Some(next) = chars.next() {
                    if escaped {
                        text.push(match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            other => other,
                        });
                        escaped = false;
                        continue;
                    }

                    if next == '\\' {
                        escaped = true;
                        continue;
                    }

                    if next == quote {
                        closed = true;
                        break;
                    }

                    text.push(next);
                }

                if escaped {
                    return Err("Unfinished escape sequence in string literal".to_string());
                }

                if !closed {
                    return Err("Unterminated string literal".to_string());
                }

                tokens.push(Token::Atom { text, quoted: true });
            }
            c if c.is_whitespace() => {}
            _ => {
                let mut atom = String::from(ch);

                while let Some(next) = chars.peek().copied() {
                    if next.is_whitespace() || next == '(' || next == ')' {
                        break;
                    }
                    atom.push(next);
                    chars.next();
                }

                tokens.push(Token::Atom {
                    text: atom,
                    quoted: false,
                });
            }
        }
    }

    Ok(tokens)
}