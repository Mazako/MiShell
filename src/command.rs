use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub trait Command {
    fn execute(&self, ctx: &mut ShellState);

    fn args(&self) -> Vec<String>;

    fn command_type(&self) -> CommandType;

    fn name(&self) -> &str;
}
