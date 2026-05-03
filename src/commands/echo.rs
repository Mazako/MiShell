use crate::command::Command;
use crate::command_type::CommandType;

pub struct Echo {
    pub(super) args: String,
}

impl Echo {
    pub(super) fn new(args: String) -> Self {
        Self { args }
    }
}

impl Command for Echo {
    fn execute(&self) {
        println!("{}", self.args);
    }

    fn args(&self) -> Vec<String> {
        vec![self.args.clone()]
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "echo"
    }
}
