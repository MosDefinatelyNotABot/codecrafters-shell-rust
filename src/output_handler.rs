use std::{fs::File, io::Write};

use crate::input_handler::TerminalResult;

pub(crate) fn handle_output(terminal_result: &TerminalResult) {
    // handle standard out
    if let Some(fname) = &terminal_result.std_out_fname {
        // print!("{}\r\n", &fname);
        match File::options()
            .append(terminal_result.is_append)
            .write(true)
            .create(true)
            .open(fname)
        {
            Ok(mut file) => {
                match file.write_all(terminal_result.standard_out.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => eprintln!("Failed to write to output file."),
                };
            }
            Err(_) => eprintln!("Failed to create output file."),
        }
    } else {
        print!("{}", terminal_result.standard_out.replace('\n', "\r\n"));
    }

    // handle standard error
    if let Some(fname) = &terminal_result.std_err_fname {
        match File::options()
            .append(terminal_result.is_append)
            .write(true)
            .create(true)
            .open(fname)
        {
            Ok(mut file) => {
                match file.write_all(terminal_result.standard_err.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => eprintln!("Failed to write to error file."),
                };
            }
            Err(_) => eprintln!("Failed to create error file."),
        }
    } else if !terminal_result.standard_err.is_empty() {
        print!("{}", terminal_result.standard_err.replace('\n', "\r\n"));
    }
}
