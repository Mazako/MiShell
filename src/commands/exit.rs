use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _ctx: &mut ShellState) {
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
