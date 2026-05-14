#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    main_loop();
}

fn main_loop() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut cmd_input = String::new();

        io::stdin()
            .read_line(&mut cmd_input)
            .expect("Failed to readline.");

        let args_input: Vec<String> = cmd_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let cmd = &args_input[0];
        let args = &args_input[1..];

        match cmd.as_str() {
            "exit" => return,
            "echo" => {
                println!("{}", args.join(" "));
                continue;
            }
            _ => {
                println!("{}: command not found", cmd);
            }
        }
    }
}
