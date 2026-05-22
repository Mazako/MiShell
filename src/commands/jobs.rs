use crate::{shell_state::ShellState, token::Input};

pub fn run(_input: &Input, ctx: &mut ShellState) {
    ctx.print_and_reap(false);
}
