use crate::command::Command;
use crate::command_type::CommandType;

pub struct Exit;

impl Command for Exit {
    fn execute(&self) {
        std::process::exit(0);
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "exit"
    }
}
