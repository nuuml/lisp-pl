use log::error;

pub fn get_inner_block(line: String) -> Vec<String> {
    let mut start = 0;
    let mut end = 0;
    let mut found_end = false;

    for (i, c) in line.char_indices() {
        if c == '(' {
            found_end = false;
            start = i;
        }
        if c == ')' && !found_end {
            end = i;
            found_end = true;
        }
    }

    if end <= start {
        return vec![];
    }

    let inner = &line[start + 1..end];
    let mut result = vec![];
    let mut current = String::new();
    let mut in_quote: Option<char> = None;

    for c in inner.chars() {
        match (c, in_quote) {
            ('"' | '\'', None) => {
                in_quote = Some(c);
                current.push(c);
            }
            (c, Some(q)) if c == q => {
                in_quote = None;
                current.push(c);
            }
            (' ' | '\t', None) => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}