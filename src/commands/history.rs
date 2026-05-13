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

        for (i, cmd) in ctx.history(n).iter().enumerate() {
            let idx = i + 1 + n;
            println!("{idx}  {cmd}")
        }
    }

    fn command_type(&self) -> crate::command_type::CommandType {
        crate::command_type::CommandType::Builtin
    }

    fn name(&self) -> &str {
        "history"
    }
}
