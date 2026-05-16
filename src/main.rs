mod find_executables;
mod parse_args;

use find_executables::find_executables;
use parse_args::quote_parser;
use std::io::{self, Write};
use std::{collections::HashMap, process::Command};

fn main() {
    // let test_strings = vec![
    //     "echo hello world".to_string(),
    //     "echo 'hello         world'".to_string(),
    //     "echo hello         world".to_string(),
    //     "echo 'hello''world'".to_string(),
    //     "echo hello''world".to_string(),
    //     "cat 'test file'".to_string(),
    //     "echo \"hello      world\"".to_string(),
    //     "echo \"hello\"\"world\"".to_string(),
    //     "echo \"hello\"world".to_string(),
    //     "echo \"hello\" \"world\"".to_string(),
    //     "echo \"Shell's test\"".to_string(),
    // ];

    // for (idx, test_string) in test_strings.iter().enumerate() {
    //     println!(
    //         "{:<3} '{:<28}' -> [{}]",
    //         idx,
    //         test_string,
    //         quote_parser(test_string).join(", ")
    //     );
    // }

    let builtins = vec!["exit", "echo", "type", "pwd", "cd"];

    let path = std::env::var("PATH").unwrap_or_default();

    let mut execs = find_executables(&path);

    for bultin in builtins {
        execs.insert(bultin.to_string(), "BUILTIN".to_string());
    }

    main_loop(&execs);
}

fn main_loop(execs: &HashMap<String, String>) {
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
        let cmd: &String = &args_input[0];
        let args: &[String] = &args_input[1..];

        // handle builtin commands
        if cmd.as_str() == "exit" {
            return;
        } else if cmd.as_str() == "echo" {
            let out_str = quote_parser(&cmd_input);
            println!("{}", out_str.join(" "));
            continue;
        } else if cmd.as_str() == "pwd" {
            println!("{}", std::env::current_dir().unwrap().to_string_lossy());
            continue;
        } else if cmd.as_str() == "type" {
            if args.len() != 1 {
                println!("error try: type <command>");
            }

            if execs.contains_key(&args[0]) {
                // check if it's a shell builtin
                if execs[&args[0]] == "BUILTIN" {
                    println!("{} is a shell builtin", args[0]);
                } else {
                    println!("{} is {}", args[0], execs[&args[0]]);
                }
            } else {
                println!("{}: not found", args[0]);
            }

            continue;
        } else if cmd.as_str() == "cd" {
            if args.len() != 1 {
                println!("error try: cd <directory>");
            } else {
                if &args[0] == "~" {
                    let home = std::env::var("HOME").unwrap_or_default();
                    std::env::set_current_dir(&home)
                        .expect(&format!("{}: No such file or directory", home));
                } else {
                    let result = std::env::set_current_dir(&args[0]);
                    if let Err(_) = result {
                        println!("{}: No such file or directory", args[0]);
                    }
                }
            }
            continue;
        } else if execs.contains_key(cmd) {
            let args_vec = quote_parser(&cmd_input);

            let output = Command::new(cmd)
                .args(args_vec)
                .output()
                .expect("{} failed to execute.");

            print!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}: not found", cmd);
            continue;
        }
    }
}
