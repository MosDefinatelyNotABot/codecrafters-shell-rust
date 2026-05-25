mod completions;
mod find_executables;
mod input_handler;
mod output_handler;
mod parse_args;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use find_executables::find_executables;
use input_handler::handle_input;

use std::collections::HashMap;
use std::io::Error;
use std::io::{Write, stdout};

use crate::{
    completions::{Completions, longest_common_prefix},
    output_handler::handle_output,
};

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
    // dropped at the end of the loop for cleanup.
    let mut _raw_mode = RawModeWrapper::new().unwrap();

    // buffer to hold user input.
    let mut input_buffer = String::new();
    let mut completion_handler = Completions::new(&execs.keys().cloned().collect::<Vec<String>>());
    let mut tab_count = 0;

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
                    tab_count += 1;
                    let completions = completion_handler.get_all_completions(&input_buffer);

                    match completions.len() {
                        0 => {
                            // no matches
                            print!("\x07");
                            stdout().flush()?;
                        }
                        1 => {
                            // unique match
                            input_buffer = format!("{} ", completions[0]);
                            execute!(stdout(), Clear(ClearType::CurrentLine))?;
                            print!("\r$ {}", input_buffer);
                            stdout().flush()?;
                        }
                        _ => {
                            let lcp = longest_common_prefix(&completions);
                            if lcp.len() > input_buffer.len() {
                                // partial completion is possible
                                input_buffer = lcp;
                                execute!(stdout(), Clear(ClearType::CurrentLine))?;
                                print!("\r$ {}", input_buffer);
                                stdout().flush()?;
                                tab_count = 0; // reset so next tab can bell/show-all if stuck
                            } else if tab_count == 1 {
                                print!("\x07");
                                stdout().flush()?;
                            } else {
                                print!("\r\n{}", completions.join("  "));
                                print!("\r\n$ {}", input_buffer);
                                stdout().flush()?;
                                tab_count = 0;
                            }
                        }
                    }
                }

                KeyCode::Enter | KeyCode::Char('j')
                    if key.code == KeyCode::Enter
                        || key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    print!("\r\n");
                    stdout().flush()?;

                    if !input_buffer.is_empty() {
                        let terminal_result = handle_input(&input_buffer, execs);

                        if terminal_result.exit_flag {
                            break;
                        }

                        handle_output(&terminal_result);
                        input_buffer.clear();
                    }

                    completion_handler.reset();
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

                KeyCode::Char(c) if !c.is_control() => {
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
