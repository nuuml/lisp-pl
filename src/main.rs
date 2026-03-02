mod input_utils;
use input_utils::get_block::get_inner_block;
use input_utils::run_command::run_command;

fn main() {
    println!("Welcome to LISP repl");
    loop {
        eprint!("𝜆:");
        let mut command = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut command).unwrap();
        let parsed = get_inner_block(command);
        if parsed.len() == 0 {
            println!("Nothing found");
        } else if parsed.len() >= 1 {
            match run_command(parsed) {
                Ok(result) => {
                    println!("{:?}", result);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
