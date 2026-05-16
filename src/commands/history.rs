use std::path::Path;

use crate::{command::Command, shell_history::ShellHistory, shell_state::ShellState, token::Input};

pub struct History {
    input: Input,
}

impl History {
    pub fn new(input: Input) -> Self {
        Self { input }
    }

    fn with_history_file(
        &self,
        ctx: &mut ShellState,
        flag: &str,
        op: impl FnOnce(&mut ShellHistory, &Path) -> std::result::Result<(), String>,
    ) {
        let Some(path_arg) = self.input.args.get(1) else {
            self.print(
                None,
                Some(&format!("history: {flag}: missing file operand")),
            );
            return;
        };
        match op(&mut ctx.history_store.borrow_mut(), Path::new(path_arg)) {
            Ok(()) => {}
            Err(msg) => self.print(None, Some(&msg)),
        }
    }
}

impl Command for History {
    fn input(&self) -> Input {
        self.input.clone()
    }

    fn execute(&self, ctx: &mut ShellState) {
        if let Some(flag) = self.input.args.first() {
            match flag.as_str() {
                "-r" => {
                    self.with_history_file(ctx, "-r", ShellHistory::read_from_file);
                    return;
                }
                "-w" => {
                    self.with_history_file(ctx, "-w", |h, p| h.write_to_file(p));
                    return;
                },
                "-a" => {
                    self.with_history_file(ctx, "-a", |h, p| h.append_to_file(p));
                    return;
                }
                _ => {}
            }
        }

        let n = self
            .input
            .args
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

        self.print(Some(&result.join("\n")), None);
    }

    fn command_type(&self) -> crate::command_type::CommandType {
        crate::command_type::CommandType::Builtin
    }

    fn name(&self) -> &str {
        "history"
    }
}
