use crate::{command_io, shell_state::ShellState, token::Input};

pub fn run(input: &Input, ctx: &ShellState) {
    command_io::print_stdout(input, &input.args(ctx).join(" "));
}
