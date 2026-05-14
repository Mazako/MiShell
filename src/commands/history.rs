use crate::{command::Command, shell_state::ShellState, token::Input};

pub struct History {
    input: Input,
}

impl History {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for History {
    fn input(&self) -> Input {
        self.input.clone()
    }

    fn execute(&self, ctx: &mut ShellState) {
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
