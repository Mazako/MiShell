mod command;
mod command_type;
mod commands;

use std::io::{self, Write};

use commands::command_from_input;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command_from_input(command.trim());
        command.execute();
    }
}