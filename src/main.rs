mod find_executables;
mod input_handler;
mod output_handler;
mod parse_args;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use find_executables::find_executables;
use input_handler::handle_input;

use std::collections::HashMap;
use std::io::Error;
use std::io::{Write, stdout};

use crate::output_handler::handle_output;

static BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

fn main() {
    let path = std::env::var("PATH").unwrap_or_default();

    let mut execs = find_executables(&path);

    for bultin in BUILTINS {
        execs.insert(bultin.to_string(), "BUILTIN".to_string());
    }

    main_loop(&execs).unwrap();
}

struct RawModeWrapper;

impl RawModeWrapper {
    fn new() -> Result<Self, Error> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeWrapper {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

fn main_loop(execs: &HashMap<String, String>) -> Result<(), Error> {
    let mut _raw_mode = RawModeWrapper::new().unwrap();
    let mut input_buffer = String::new();
    // let mut terminal_result = TerminalResult::default();

    // print first line marker.
    print!("$ ");
    stdout().flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Tab => {
                    print!("<TAB!>")
                }
                KeyCode::Enter => {
                    // does a thing here
                    let terminal_result = handle_input(&input_buffer, execs);

                    if terminal_result._exit_flag {
                        break;
                    }
                    print!("\r\n");
                    handle_output(&terminal_result);

                    // print!("\r\n\"{}\"", input_buffer);
                    input_buffer.clear();
                    print!("\r$ ");
                    stdout().flush()?;
                }
                KeyCode::Backspace if input_buffer.pop().is_some() => {
                    execute!(stdout(), cursor::MoveLeft(1))?;
                    print!(" ");
                    execute!(stdout(), cursor::MoveLeft(1))?;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                KeyCode::Char(c) => {
                    input_buffer.push(c);
                    print!("{}", c);
                    stdout().flush()?;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
