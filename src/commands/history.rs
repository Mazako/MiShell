use std::path::Path;

use crate::{command_io, shell_history::ShellHistory, shell_state::ShellState, token::Input};

fn with_history_file(
    input: &Input,
    ctx: &mut ShellState,
    flag: &str,
    op: impl FnOnce(&mut ShellHistory, &Path) -> std::result::Result<(), String>,
) {
    let args = input.args(ctx);
    let Some(path_arg) = args.get(1) else {
        command_io::print_stderr(input, &format!("history: {flag}: missing file operand"));
        return;
    };
    match op(&mut ctx.history_store.borrow_mut(), Path::new(path_arg)) {
        Ok(()) => {}
        Err(msg) => command_io::print_stderr(input, &msg),
    }
}

pub fn run(input: &Input, ctx: &mut ShellState) {
    let args = input.args(ctx);
    if let Some(flag) = args.first() {
        match flag.as_str() {
            "-r" => {
                with_history_file(input, ctx, "-r", ShellHistory::read_from_file);
                return;
            }
            "-w" => {
                with_history_file(input, ctx, "-w", |h, p| h.write_to_file(p));
                return;
            }
            "-a" => {
                with_history_file(input, ctx, "-a", |h, p| h.append_to_file(p));
                return;
            }
            _ => {}
        }
    }

    let n = args
        .first()
        .map(|f| f.parse::<usize>().unwrap_or(0))
        .unwrap_or(0);

    let len = ctx.history_len();
    let take = if n == 0 { len } else { n.min(len) };
    let start_idx = len.saturating_sub(take) + 1;
    let mut result = Vec::new();

    for (i, cmd) in ctx.history_last_n(take).iter().enumerate() {
        result.push(format!("{}  {}", start_idx + i, cmd));
    }

    command_io::print_stdout(input, &result.join("\n"));
}
