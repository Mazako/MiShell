use std::path::PathBuf;
use std::process::Child;

use crate::{command_io, shell_state::ShellState, token::Input};

pub fn run(input: &Input, _path: &PathBuf, ctx: &ShellState) {
    let mut command = std::process::Command::new(&input.command);
    command.args(input.args(ctx));
    command_io::apply_redirects(input, &mut command);
    command.status().unwrap();
}

pub fn spawn_background(input: &Input, ctx: &ShellState) -> Child {
    std::process::Command::new(&input.command)
        .args(input.args(ctx))
        .spawn()
        .unwrap()
}
