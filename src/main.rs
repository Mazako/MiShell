mod command;
mod command_type;
mod commands;
mod shell_state;

use std::io::{self, Write};

use commands::command_from_input;
use shell_state::ShellState;

fn main() {
    let mut ctx = ShellState::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command_from_input(command.trim(), &ctx);
        command.execute(&mut ctx);
    }
}