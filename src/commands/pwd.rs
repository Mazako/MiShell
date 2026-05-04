use crate::{command::Command, command_type::CommandType, shell_state::ShellState};


pub struct Pwd;

impl Command for Pwd {
    fn execute(&self, ctx: &mut ShellState) {
        println!("{}", ctx.cwd.to_str().unwrap())
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "pwd"
    }
}