#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, os::unix::fs::MetadataExt};

fn main() {
    let builtins = vec!["exit", "echo", "type"];

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
                    println!("error try: type <command>");
                    continue;
                }
                if execs.contains_key(&args[0]) {
                    // check if it's a shell builtin
                    if execs[&args[0]] == "BUILTIN" {
                        println!("{} is a shell builtin", args[0]);
                    } else {
                        println!("{} is {}", args[0], execs[&args[0]]);
                    }
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

fn find_executables(path: &str) -> HashMap<String, String> {
    let mut executables = HashMap::new();

    // implement crawling logic here.

    // get all the elements in the path
    let dirs_in_path = path
        .split(":")
        .into_iter()
        .filter(|dir| !dir.is_empty())
        .collect::<Vec<_>>();

    // for each element
    let subdirs_and_execs = dirs_in_path
        .iter()
        .flat_map(|dir| std::fs::read_dir(dir).unwrap())
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.metadata().unwrap().mode() == 0o001)
        .collect::<Vec<_>>();

    // split into executables and subdirectories
    let execs = subdirs_and_execs
        .iter()
        .filter(|entry| entry.metadata().unwrap().mode() == 0o001);

    for exec in execs {
        let name = exec.file_name().to_string_lossy().into_owned();
        let exec_path = exec.path().to_string_lossy().into_owned();
        executables.insert(name, exec_path);
    }

    executables
}
