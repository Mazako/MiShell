use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::Input;

pub struct Exit {
    input: Input,
}

impl Exit {
    pub(super) fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for Exit {
    fn execute(&self, ctx: &mut ShellState) {
        ctx.running = false;
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "exit"
    }
}
