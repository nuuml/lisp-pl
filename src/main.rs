mod input_utils;
use input_utils::run_command::run_command;

fn main() {
    println!("Welcome to LISP repl");
    loop {
        eprint!("𝜆:");
        let mut command = String::new();
        let stdin = std::io::stdin();
        let bytes_read = stdin.read_line(&mut command).unwrap();
        if bytes_read == 0 {
            break;
        }
        let source = command.trim();

        if source.is_empty() {
            println!("Nothing found");
            continue;
        }

        match run_command(source) {
            Ok(result) => {
                let rendered = result.to_string();
                if !rendered.is_empty() {
                    println!("{rendered}");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
