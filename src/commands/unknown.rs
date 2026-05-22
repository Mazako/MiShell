use crate::{command_io, shell_state::ShellState, token::Input};

pub fn run(input: &Input, _ctx: &ShellState) {
    command_io::print_stderr(input, &format!("{}: command not found", input.command));
}
