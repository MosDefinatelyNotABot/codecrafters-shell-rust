#[allow(unused_imports)]
use std::io::{self, Write};

use bytes::buf;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let builtins = vec!["exit", "echo", "type"];

    main_loop(builtins);
}

fn main_loop(builtins: Vec<&str>) {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // string buffer for user input
        let mut cmd_input = String::new();

        // read user input from stdin
        io::stdin()
            .read_line(&mut cmd_input)
            .expect("Failed to readline.");

        // parse user input into command and arguments
        let args_input: Vec<String> = cmd_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // short hand
        let cmd = &args_input[0];
        let args = &args_input[1..];

        // handle builtin commands
        match cmd.as_str() {
            "exit" => return,
            "echo" => {
                println!("{}", args.join(" "));
                continue;
            }
            "type" => {
                if args.len() != 1 {
                    println!("type: usage: type <command>");
                    continue;
                }

                if builtins.contains(&args[0].as_str()) {
                    println!("{} is a shell builtin", args[0]);
                    continue;
                } else {
                    println!("{}: not found", args[0]);
                    continue;
                }
            }
            _ => {
                println!("{}: command not found", cmd);
            }
        }
    }
}
