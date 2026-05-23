// handle_input is run each time Enter is pressed.

use std::{collections::HashMap, process::Command};

use crate::parse_args::term_tokenizer;

#[derive(Default)]
pub(crate) struct TerminalResult {
    pub std_out_fname: Option<String>,
    pub std_err_fname: Option<String>,
    pub standard_out: String,
    pub standard_err: String,
    pub is_append: bool,
    pub _exit_flag: bool,
    pub _exit_code: i32,
}

impl TerminalResult {
    fn exit() -> Self {
        Self {
            std_out_fname: None,
            std_err_fname: None,
            standard_out: String::new(),
            standard_err: String::new(),
            is_append: false,
            _exit_flag: true,
            _exit_code: 0,
        }
    }
}

pub(crate) fn handle_input(input_buffer: &str, execs: &HashMap<String, String>) -> TerminalResult {
    let (cmd, mut args) = term_tokenizer(input_buffer);

    // return values in TerminalResult object
    let mut std_out_fname: Option<String> = None;
    let mut std_err_fname: Option<String> = None;
    let mut is_append = false;

    let mut standard_out = String::new();
    let mut standard_err = String::new();

    // check for output redirection
    if args.contains(&">".to_string())
        || args.contains(&"1>".to_string())
        || args.contains(&"2>".to_string())
        || args.contains(&">>".to_string())
        || args.contains(&"1>>".to_string())
        || args.contains(&"2>>".to_string())
    {
        let pipe_index = args
            .iter()
            .position(|arg| {
                arg == ">"
                    || arg == "1>"
                    || arg == "2>"
                    || arg == ">>"
                    || arg == "1>>"
                    || arg == "2>>"
            })
            // should not ever be None
            .expect("No output redirection operator found.");

        let is_sent_to_err = (args[pipe_index] == "2>") || (args[pipe_index] == "2>>");
        is_append = (args[pipe_index] == ">>")
            || (args[pipe_index] == "1>>")
            || (args[pipe_index] == "2>>");

        // output_file = Some(args.get(pipe_index + 1).expect("").clone());
        match args.get(pipe_index + 1) {
            Some(file) => {
                if is_sent_to_err {
                    std_err_fname = Some(file.clone());
                } else {
                    std_out_fname = Some(file.clone());
                }
            }
            None => standard_err = "No output file specified.".to_string(),
        }

        args = args[..pipe_index].to_vec();
    };

    // handle builtin commands
    if cmd.as_str() == "exit" {
        return TerminalResult::exit();
    } else if cmd.as_str() == "echo" {
        // print args to stdout
        standard_out = format!("{}\n", args.join(" ").trim());
    } else if cmd.as_str() == "pwd" {
        // prints current directory to stdout
        match std::env::current_dir() {
            Ok(dir) => standard_out = format!("{}\n", dir.to_string_lossy()),
            Err(e) => standard_err = format!("{}\n", e),
        }
    } else if cmd.as_str() == "type" {
        // prints the type of the command to stdout
        if args.len() != 1 {
            standard_err = "error try: type <command>\n".to_string();
        }

        if execs.contains_key(&args[0]) {
            // check if it's a shell builtin
            if execs[&args[0]] == "BUILTIN" {
                standard_out = format!("{} is a shell builtin\n", args[0]);
            } else {
                standard_out = format!("{} is {}\n", args[0], execs[&args[0]]);
            }
        } else {
            standard_err = format!("{}: not found\n", args[0]);
        }
    } else if cmd.as_str() == "cd" {
        // changes directory to the specified path
        if args.len() != 1 {
            standard_err = "error try: cd <directory>".to_string();
        } else {
            if &args[0] == "~" {
                match std::env::var("HOME") {
                    Ok(home) => match std::env::set_current_dir(&home) {
                        Ok(_) => {}
                        Err(_) => {
                            standard_err = "Failed to set current directory to HOME.\n".to_string()
                        }
                    },
                    Err(_) => standard_err = "HOME not set\n".to_string(),
                }
            } else {
                match std::env::set_current_dir(&args[0]) {
                    Ok(_) => {}
                    Err(_) => {
                        standard_err = format!("{}: {}: No such file or directory\n", cmd, args[0])
                    }
                }
            }
        }
    } else if execs.contains_key(&cmd) {
        // executes shell command with args
        match Command::new(&cmd).args(args).output() {
            Ok(output) => {
                standard_out = String::from_utf8_lossy(&output.stdout).to_string();
                standard_err = String::from_utf8_lossy(&output.stderr).to_string();
            }
            Err(err) => standard_err = format!("{} failed to execute: {}\n", cmd, err).to_string(),
        }
    } else {
        // error message if command not found
        standard_err = format!("{}: command not found\n", cmd).to_string();
    }

    // Bundle and return the results
    TerminalResult {
        std_out_fname,
        std_err_fname,
        standard_out,
        standard_err,
        is_append,
        _exit_flag: false,
        _exit_code: 0,
    }
}
