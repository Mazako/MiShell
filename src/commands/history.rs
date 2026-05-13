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
        todo!()
    }

    fn command_type(&self) -> crate::command_type::CommandType {
        crate::command_type::CommandType::Builtin
    }

    fn name(&self) -> &str {
        "history"
    }
}