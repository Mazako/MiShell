use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;
use crate::token::Input;

pub struct UnknownCommand {
    pub(super) input: Input,
}

impl UnknownCommand {
    pub(super) fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Command for UnknownCommand {
    fn execute(&self, _ctx: &mut ShellState) {
        self.print(
            None,
            Some(&format!("{}: command not found", self.input.command)),
        );
    }

    fn input(&self) -> Input {
        self.input.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Unrecognized
    }

    fn name(&self) -> &str {
        &self.input.command
    }
}
