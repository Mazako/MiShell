use crate::command::Command;
use crate::command_type::CommandType;

pub struct UnknownCommand {
    pub(super) command: String,
}

impl UnknownCommand {
    pub(super) fn new(command: String) -> Self {
        Self { command }
    }
}

impl Command for UnknownCommand {
    fn execute(&self) {
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
