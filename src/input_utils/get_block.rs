use log::error;

pub fn get_inner_block(line: String) -> Vec<String> {
    let mut start = 0;
    let mut end = 0;
    let mut found_end = false;
    for (i, c) in line.char_indices() {
        if (c == '(') {
            found_end = false;
            start = i;
        }
        if (c == ')' && !found_end) {
            end = i;
            found_end = true;
        }
    }
    let mut result = vec![];
    if (end - start > 0) {
        println!("{}", &line[start..end +1 ]);
        result = line[start+1..end].split(" ").map(|s| s.to_string()).collect();
        return result
    }
    result
    
}