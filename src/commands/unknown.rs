use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct UnknownCommand {
    pub(super) command: String,
}

impl UnknownCommand {
    pub(super) fn new(command: String) -> Self {
        Self { command }
    }
}

impl Command for UnknownCommand {
    fn execute(&self, _ctx: &mut ShellState) {
        println!("{}: command not found", self.command);
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Unrecognized
    }

    fn name(&self) -> &str {
        &self.command
    }
}
