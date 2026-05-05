use crate::command::Command;
use crate::command_type::CommandType;
use crate::shell_state::ShellState;

pub struct Echo {
    pub(super) args: Vec<String>,
}

impl Echo {
    pub(super) fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for Echo {
    fn execute(&self, _ctx: &mut ShellState) {
        println!("{}", self.args.join(" "));
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "echo"
    }
}
