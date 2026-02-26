mod input_utils;
use input_utils::get_block::get_inner_block;

fn main() {
    println!("Welcome to LISP repl");
    println!("Please enter command:");
    let mut command = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut command).unwrap();
    println!("Command: {}", get_inner_block(command)[0]);
}
