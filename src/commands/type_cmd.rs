use crate::command::Command;
use crate::command_type::CommandType;

pub struct Type {
    pub(super) args: Vec<String>,
}

impl Type {
    pub(super) fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for Type {
    fn execute(&self) {
        let args = self.args();
        if args.is_empty() {
            println!();
            return;
        }
        let cmd = super::command_from_input(&args[0]);
        match cmd.command_type() {
            CommandType::Builtin => println!("{} is a shell builtin", cmd.name()),
            CommandType::Executable => print!("{} is a executable", cmd.name()),
            CommandType::Unrecognized => println!("{}: not found", cmd.name()),
        }
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn name(&self) -> &str {
        "type"
    }
}
