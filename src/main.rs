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

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to readline.");

        let cmd = cmd.trim();

        match cmd {
            "exit" => return,
            _ => {
                println!("{}: command not found", cmd);
            }
        }
    }
}
